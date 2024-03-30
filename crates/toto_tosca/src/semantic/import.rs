use std::collections::HashMap;

use petgraph::{
    algo::toposort,
    visit::{EdgeFiltered, EdgeRef, NodeFiltered, NodeRef},
    Direction::Outgoing,
};
use toto_parser::EntityParser;

use crate::{grammar::parser::ToscaGrammar, ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Importer {
    existing_urls: HashMap<url::Url, toto_ast::GraphHandle>,
}

impl Importer {
    pub fn new() -> Self {
        Self {
            existing_urls: HashMap::new(),
        }
    }

    pub fn add_file<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        if let Some(file_handle) = self.existing_urls.get(&uri) {
            return *file_handle;
        }

        let mut doc = toto_yaml::FileEntity::from_url(uri.clone());
        doc.fetch().unwrap();

        let doc_handle = ast.add_node(doc.into());
        let doc_root = toto_yaml::YamlParser::parse(doc_handle, ast).unwrap();
        let file_handle = ToscaGrammar::parse(doc_root, ast).unwrap();

        self.existing_urls.insert(uri.clone(), file_handle);
        self.import_files(uri, file_handle, ast);

        file_handle
    }

    fn import_files<E, R>(
        &mut self,
        uri: &url::Url,
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.edges(file_handle)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Import(_)) => Some(e.target()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|import_def| {
                let import_uri = ast
                    .edges_directed(import_def, Outgoing)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::Url) => Some(e.target()),
                        _ => None,
                    })
                    .and_then(|u| toto_yaml::as_string(u, ast))
                    .and_then(|u| url::Url::parse(&u).or(uri.join(&u)).ok());

                if import_uri.is_none() {
                    todo!("handle profile");
                }
                let import_uri = import_uri.unwrap();

                let imported_file = self.add_file(&import_uri, ast);
                ast.add_edge(
                    file_handle,
                    imported_file,
                    crate::Relation::ImportFile.into(),
                );
                ast.add_edge(
                    import_def,
                    imported_file,
                    crate::Relation::ImportTarget.into(),
                );
            });
    }

    pub fn topo_iter_imports<E, R>(
        ast: &toto_ast::AST<E, R>,
    ) -> Result<impl Iterator<Item = toto_ast::GraphHandle>, toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file_graph = EdgeFiltered::from_fn(&*ast, |e| {
            matches!(e.weight().as_tosca(), Some(crate::Relation::ImportFile))
        });
        let file_graph = NodeFiltered::from_fn(&file_graph, |n| {
            matches!(
                ast.node_weight(n.id()).unwrap().as_tosca(),
                Some(crate::Entity::File)
            )
        });

        toposort(&file_graph, None)
            .map_err(|err| err.node_id())
            .map(|v| v.into_iter().rev())
    }

    pub fn import_types<E, R>(file_handle: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.edges(file_handle)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Import(_)) => Some(e.target()),
                _ => None,
            })
            .filter_map(|import_def| {
                ast.edges_directed(import_def, Outgoing)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::ImportTarget) => Some((import_def, e.target())),
                        _ => None,
                    })
            })
            .map(|(import_def, target_file)| {
                let ns = ast
                    .edges_directed(import_def, Outgoing)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::ImportNamespace) => Some(e.target()),
                        _ => None,
                    })
                    .and_then(|u| toto_yaml::as_string(u, ast));

                (target_file, ns)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(target_file, ns)| {
                ast.edges_directed(target_file, Outgoing)
                    .filter_map(|e| {
                        e.weight()
                            .as_tosca()
                            .and_then(|n| match n {
                                crate::Relation::Type(type_name) => Some(crate::Relation::Type(
                                    [ns.as_slice(), &[type_name.clone()]].concat().join(":"),
                                )),
                                _ => None,
                            })
                            .map(|rel| (e.target(), rel))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|(target_def, rel)| {
                        if ast
                            .edges_connecting(file_handle, target_def)
                            .find(|e| *e.weight().as_tosca().unwrap() == rel)
                            .is_some()
                        {
                            return;
                        }
                        ast.add_edge(file_handle, target_def, rel.into());
                    });
            });
    }
}
