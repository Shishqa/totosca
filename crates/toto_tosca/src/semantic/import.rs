use std::{
    collections::{HashMap, VecDeque},
    fmt::format,
};

use petgraph::{
    algo::toposort,
    data::Build,
    visit::{EdgeFiltered, EdgeRef, IntoEdgesDirected, NodeFiltered, NodeRef},
    Direction::{Incoming, Outgoing},
};
use toto_parser::{add_with_loc, EntityParser};
use toto_yaml::YamlParser;

use crate::{grammar::parser::ToscaGrammar, ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Importer;

impl Importer {
    pub fn deduce_url<E, R>(
        file_handle: toto_ast::GraphHandle,
        ast: &toto_ast::AST<E, R>,
    ) -> url::Url
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file = ast
            .edges(file_handle)
            .find_map(|e| match e.weight().as_file() {
                Some(_) => Some(e.target()),
                _ => None,
            })
            .unwrap();
        let file = ast.node_weight(file).unwrap().as_file().unwrap();

        file.url.clone()
    }

    pub fn import_files<E, R>(
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Vec<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        dbg!(ast.node_weight(file_handle));

        let root_url = Self::deduce_url(file_handle, ast);

        let mut existing_urls = HashMap::<url::Url, toto_ast::GraphHandle>::from_iter([(
            root_url.clone(),
            file_handle,
        )]);

        let mut to_import =
            VecDeque::<(url::Url, toto_ast::GraphHandle)>::from_iter([(root_url, file_handle)]);

        while !to_import.is_empty() {
            let (root_url, file_handle) = to_import.pop_front().unwrap();
            ast.edges(file_handle)
                .filter_map(|e| match e.weight().as_tosca() {
                    Some(crate::Relation::Import(_)) => Some(e.target()),
                    _ => None,
                })
                .filter_map(|import_def| {
                    ast.edges_directed(import_def, Outgoing)
                        .find_map(|e| match e.weight().as_tosca() {
                            Some(crate::Relation::Url) => Some(e.target()),
                            _ => None,
                        })
                        .and_then(|u| toto_yaml::as_string(u, ast))
                        .and_then(|u| url::Url::parse(&u).or(root_url.join(&u)).ok())
                        .map(|u| (import_def, u))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(import_def, url)| {
                    existing_urls
                        .get(&url)
                        .cloned()
                        .or_else(|| {
                            let mut doc = toto_yaml::FileEntity::from_url(url.clone());
                            if let Err(err) = doc.fetch() {
                                add_with_loc(
                                    toto_parser::ParseError::Custom(err.to_string()),
                                    import_def,
                                    ast,
                                );
                                return None;
                            }

                            let doc_handle = ast.add_node(doc.into());
                            let doc_root = YamlParser::parse(doc_handle, ast).unwrap();
                            let imported_file = ToscaGrammar::parse(doc_root, ast).unwrap();

                            to_import.push_back((url.clone(), imported_file));
                            existing_urls.insert(url, imported_file);

                            Some(imported_file)
                        })
                        .map(|imported_file| {
                            ast.add_edge(
                                import_def,
                                imported_file,
                                crate::Relation::ImportFile.into(),
                            );
                            imported_file
                        });
                });
        }

        let file_graph = EdgeFiltered::from_fn(&*ast, |e| {
            matches!(
                e.weight().as_tosca(),
                Some(crate::Relation::ImportFile | crate::Relation::Import(_))
            )
        });
        let file_graph = NodeFiltered::from_fn(&file_graph, |n| {
            matches!(
                ast.node_weight(n.id()).unwrap().as_tosca(),
                Some(crate::Entity::File | crate::Entity::Definition)
            )
        });

        toposort(&file_graph, None)
            .map_err(|err| {
                add_with_loc(
                    toto_parser::ParseError::Custom("circular import detected".to_string()),
                    err.node_id(),
                    ast,
                );
            })
            .ok()
            .unwrap_or(vec![file_handle])
            .into_iter()
            .filter(|n| {
                matches!(
                    ast.node_weight(*n).unwrap().as_tosca(),
                    Some(crate::Entity::File)
                )
            })
            .rev()
            .collect()
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
                        Some(crate::Relation::ImportFile) => Some((import_def, e.target())),
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
                                crate::Relation::NodeType(type_name) => {
                                    Some(crate::Relation::NodeType(
                                        [ns.as_slice(), &[type_name.clone()]].concat().join(":"),
                                    ))
                                }
                                _ => None,
                            })
                            .map(|rel| (e.target(), rel))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|(target_def, rel)| {
                        ast.add_edge(file_handle, target_def, rel.into());
                    });
            });
    }
}
