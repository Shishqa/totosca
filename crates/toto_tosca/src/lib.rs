pub mod grammar;
pub mod models;
pub mod semantic;

use anyhow::Ok;
use grammar::{parser::ToscaGrammar, v1_3::Tosca1_3, v2_0::Tosca2_0, ToscaDefinitionsVersion};
pub use models::*;
use petgraph::{visit::EdgeRef, Direction};
use semantic::{Derive, FileStorage, Importer, Lookup};
use toto_parser::{add_with_loc, ParseError};

#[derive(Default)]
pub struct ToscaParser {
    files: FileStorage,
}

impl ToscaParser {
    pub fn new() -> Self {
        Self {
            files: FileStorage::new(),
        }
    }

    pub fn get_files(&self) -> impl Iterator<Item = &url::Url> {
        self.files.get_files()
    }

    pub fn parse<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> anyhow::Result<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.clear();
        self.files.clear();

        let doc_root = self.parse_file(uri, ast)?;

        Importer::import_all_types(ast);
        Lookup::lookup(ast);
        Derive::inherit_all_definitions(ast);

        Ok(doc_root)
    }

    pub fn parse_file<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> anyhow::Result<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let doc_root = self.files.add_file(uri, ast)?;

        let yaml_root = toto_yaml::YamlParser::parse(doc_root, ast);
        if let Err(err) = yaml_root {
            Self::report_yaml_error(err.to_string(), doc_root, ast);
            return Ok(doc_root);
        }
        let yaml_root = yaml_root.unwrap();

        let Some(tosca_version) = ToscaGrammar::get_tosca_version(yaml_root, ast) else {
            return Ok(doc_root);
        };

        match tosca_version.1.as_str() {
            Tosca1_3::<E, R>::NAME => {
                self.parse_versioned::<E, R, Tosca1_3<E, R>>(uri, yaml_root, ast)
            }
            Tosca2_0::<E, R>::NAME => {
                self.parse_versioned::<E, R, Tosca2_0<E, R>>(uri, yaml_root, ast)
            }
            _ => {
                add_with_loc(
                    ParseError::Custom("unknown tosca version".to_string()),
                    tosca_version.0,
                    ast,
                );
                None
            }
        };

        Ok(doc_root)
    }

    fn parse_versioned<E, R, V>(
        &mut self,
        uri: &url::Url,
        yaml_root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
        V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
    {
        let doc = toto_yaml::FileEntity {
            url: url::Url::parse(format!("builtin://{}", V::NAME).as_str()).unwrap(),
            content: None,
        };
        let builtin_root = ast.add_node(doc.into());
        let builtin_handle = ast.add_node(crate::Entity::File(crate::FileEntity).into());
        ast.add_edge(
            builtin_handle,
            builtin_root,
            toto_yaml::FileRelation(0).into(),
        );

        V::add_builtins(builtin_handle, ast);

        self.parse_file_versioned::<E, R, V>(uri, yaml_root, builtin_root, builtin_handle, ast)
    }

    fn find_file<E, R>(
        doc_root: toto_ast::GraphHandle,
        ast: &toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.edges_directed(doc_root, Direction::Incoming)
            .filter_map(|e| {
                if e.weight().as_file().is_some() {
                    Some(e.source())
                } else {
                    None
                }
            })
            .find(|n| {
                matches!(
                    ast.node_weight(*n).unwrap().as_tosca(),
                    Some(crate::Entity::File(_))
                )
            })
    }

    fn parse_file_versioned<E, R, V>(
        &mut self,
        uri: &url::Url,
        yaml_root: toto_ast::GraphHandle,
        builtin_root: toto_ast::GraphHandle,
        builtin_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
        V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
    {
        let file_handle = V::parse(yaml_root, ast)?;

        let import_entity = ast.add_node(crate::Entity::from(crate::ImportEntity).into());

        ast.add_edge(
            file_handle,
            import_entity,
            crate::Relation::from(crate::ImportRelation(1000000)).into(),
        );
        ast.add_edge(
            file_handle,
            builtin_handle,
            crate::Relation::from(crate::ImportFileRelation).into(),
        );
        ast.add_edge(
            import_entity,
            builtin_root,
            crate::Relation::from(crate::ImportTargetRelation).into(),
        );

        for (import_uri, import_def) in Importer::iter_imports(uri, file_handle, ast) {
            if let Some(handle) = self.files.get_file(&import_uri) {
                if let Some(parsed_handle) = Self::find_file(handle, ast) {
                    ast.add_edge(
                        file_handle,
                        parsed_handle,
                        crate::Relation::from(crate::ImportFileRelation).into(),
                    );
                }
                ast.add_edge(
                    import_def,
                    handle,
                    crate::Relation::from(crate::ImportTargetRelation).into(),
                );
                continue;
            }

            let doc_root = self.files.add_file(&import_uri, ast);
            if let Err(err) = doc_root {
                toto_parser::add_with_loc(
                    toto_parser::ParseError::Custom(err.to_string()),
                    import_def,
                    ast,
                );
                continue;
            }
            let doc_root = doc_root.unwrap();
            ast.add_edge(
                import_def,
                doc_root,
                crate::Relation::from(crate::ImportTargetRelation).into(),
            );

            let yaml_root = toto_yaml::YamlParser::parse(doc_root, ast);
            if let Err(err) = yaml_root {
                Self::report_yaml_error(err.to_string(), doc_root, ast);
                continue;
            }
            let yaml_root = yaml_root.unwrap();

            let Some(tosca_version) = ToscaGrammar::get_tosca_version(yaml_root, ast) else {
                continue;
            };

            if tosca_version.1 != V::NAME {
                toto_parser::add_with_loc(
                    toto_parser::ParseError::Custom(
                        "can't import file of different tosca version".to_string(),
                    ),
                    import_def,
                    ast,
                );
                continue;
            }

            if let Some(target_handle) = self.parse_file_versioned::<E, R, V>(
                &import_uri,
                yaml_root,
                builtin_root,
                builtin_handle,
                ast,
            ) {
                ast.add_edge(
                    file_handle,
                    target_handle,
                    crate::Relation::from(crate::ImportFileRelation).into(),
                );
            }
        }

        Some(file_handle)
    }

    fn report_yaml_error<E, R>(
        err: String,
        doc_root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let doc = ast[doc_root].as_file().unwrap();

        let err_lines = err.splitn(4, "\n").collect::<Vec<_>>();
        let err_pos = err_lines
            .iter()
            .nth(2)
            .and_then(|s| {
                s.split_once(":")
                    .map(|s| (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap()))
            })
            .map(|(lineno, charno)| {
                toto_yaml::from_lc(doc.content.as_ref().unwrap().as_str(), lineno, charno)
            })
            .unwrap_or_default();

        let err_handle = ast.add_node(
            toto_parser::ParseError::Custom(
                err_lines
                    .first()
                    .map(|e| e.strip_suffix(": ").unwrap_or(e).to_string())
                    .unwrap_or(err),
            )
            .into(),
        );
        ast.add_edge(
            err_handle,
            doc_root,
            toto_yaml::FileRelation(err_pos).into(),
        );
    }
}
