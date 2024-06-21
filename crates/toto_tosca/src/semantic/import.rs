use std::collections::HashMap;

use petgraph::{
    algo::toposort,
    data::DataMap,
    visit::{EdgeFiltered, EdgeRef, NodeFiltered, NodeRef},
    Direction::Outgoing,
};
use toto_parser::{add_with_loc, ParseError};

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

#[derive(Default)]
pub struct FileStorage {
    existing_urls: HashMap<url::Url, toto_ast::GraphHandle>,
}

impl FileStorage {
    pub fn new() -> Self {
        Self {
            existing_urls: HashMap::new(),
        }
    }

    pub fn add_file<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> anyhow::Result<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        if let Some(file_handle) = self.get_file(uri) {
            return Ok(file_handle);
        }

        let mut doc = toto_yaml::FileEntity::from_url(uri.clone());
        doc.fetch()?;

        let doc_handle = ast.add_node(doc.into());
        self.existing_urls.insert(uri.clone(), doc_handle);
        Ok(doc_handle)
    }

    pub fn has_file(&self, uri: &url::Url) -> bool {
        self.existing_urls.contains_key(uri)
    }

    pub fn get_files(&self) -> impl Iterator<Item = &url::Url> {
        self.existing_urls.keys()
    }

    pub fn get_file(&self, uri: &url::Url) -> Option<toto_ast::GraphHandle> {
        self.existing_urls.get(uri).copied()
    }

    pub fn clear(&mut self) {
        self.existing_urls.clear()
    }
}

pub struct Importer;

impl Importer {
    pub fn import_all_types<E, R>(ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let _ = Self::topo_iter_imports(ast)
            .map_err(|e| {
                add_with_loc(
                    ParseError::Custom("circular import detected".to_string()),
                    e,
                    ast,
                );
            })
            .map(|imports| {
                imports.for_each(|file_handle| {
                    Self::import_types(file_handle, ast);
                });
            });
    }

    pub fn iter_imports<E, R>(
        uri: &url::Url,
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> impl Iterator<Item = (url::Url, toto_ast::GraphHandle)>
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
                        Some(crate::Relation::ImportUrl(_)) => Some(e.target()),
                        _ => None,
                    })
                    .and_then(|u| toto_yaml::as_string(u, ast))
                    .and_then(|u| url::Url::parse(&u.0).or(uri.join(&u.0)).ok())
                    .zip(Some(import_def))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn topo_iter_imports<E, R>(
        ast: &toto_ast::AST<E, R>,
    ) -> Result<impl Iterator<Item = toto_ast::GraphHandle>, toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file_graph = EdgeFiltered::from_fn(ast, |e| {
            matches!(e.weight().as_tosca(), Some(crate::Relation::ImportFile(_)))
        });
        let file_graph = NodeFiltered::from_fn(&file_graph, |n| {
            matches!(
                ast.node_weight(n.id()).unwrap().as_tosca(),
                Some(crate::Entity::File(_))
            )
        });

        toposort(&file_graph, None)
            .map_err(|err| err.node_id())
            .map(|v| v.into_iter().rev())
    }

    fn import_types<E, R>(file_handle: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>)
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
                        Some(crate::Relation::ImportTarget(_)) => Some((import_def, e.target())),
                        _ => None,
                    })
            })
            .map(|(import_def, doc_root)| {
                let ns = ast
                    .edges_directed(import_def, Outgoing)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::ImportNamespace(_)) => Some(e.target()),
                        _ => None,
                    })
                    .and_then(|u| toto_yaml::as_string(u, ast).cloned())
                    .map(|u| u.0);

                (doc_root, ns)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(doc_root, ns)| {
                let Some(target_file) = ast
                    .edges_directed(doc_root, petgraph::Direction::Incoming)
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
                else {
                    return;
                };

                ast.edges_directed(target_file, Outgoing)
                    .filter_map(|e| {
                        e.weight()
                            .as_tosca()
                            .and_then(|n| match n {
                                crate::Relation::Type(type_name) => {
                                    Some(crate::Relation::Type(crate::TypeRelation(
                                        [ns.as_slice(), &[type_name.0.clone()]].concat().join(":"),
                                    )))
                                }
                                _ => None,
                            })
                            .map(|rel| (e.target(), rel))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|(target_def, rel)| {
                        if ast
                            .edges_connecting(file_handle, target_def)
                            .any(|e| *e.weight().as_tosca().unwrap() == rel)
                        {
                            return;
                        }
                        ast.add_edge(file_handle, target_def, rel.into());
                    });
            });
    }
}
