use std::collections::{HashMap, HashSet};

use petgraph::visit::EdgeRef;
use toto_parser::add_with_loc;
use toto_yaml::{FileEntity, YamlParser};

use crate::{grammar::parser::ToscaParser, ToscaCompatibleEntity, ToscaCompatibleRelation};

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
        let yaml = ast
            .edges(file_handle)
            .find_map(|e| match e.weight().as_parse_loc() {
                Some(_) => Some(e.target()),
                _ => None,
            })
            .unwrap();
        let file = ast
            .edges(yaml)
            .find_map(|e| match e.weight().as_file() {
                Some(_) => Some(e.target()),
                _ => None,
            })
            .unwrap();
        let file = ast.node_weight(file).unwrap().as_file().unwrap();

        file.url.clone()
    }

    pub fn find_urls<E, R>(ast: &toto_ast::AST<E, R>) -> HashMap<url::Url, toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.node_indices()
            .filter_map(|n| match ast.node_weight(n).unwrap().as_file() {
                Some(file) => Some((file.url.clone(), n)),
                _ => None,
            })
            .collect::<HashMap<_, _>>()
    }
}

impl<E, R> toto_ast::EntityParser<E, R> for Importer
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        dbg!(ast.node_weight(file_handle));

        let root_url = Self::deduce_url(file_handle, ast);

        let existing_urls = Self::find_urls(ast);

        dbg!(&existing_urls);
        dbg!(&root_url);

        ast.edges(file_handle)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Import(_)) => Some(ast.edges(e.target())),
                _ => None,
            })
            .flatten()
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Url) => Some(e.target()),
                _ => None,
            })
            .filter_map(|n| match toto_yaml::as_string(n, ast) {
                Some(import_url) => Some((n, import_url.to_owned())),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(n, import_url)| {
                let import_url = url::Url::parse(&import_url)
                    .or(root_url.join(&import_url))
                    .unwrap();

                dbg!(&import_url);

                if let Some(existing_handle) = existing_urls.get(&import_url) {
                    ast.add_edge(n, *existing_handle, crate::Relation::ImportFile.into());
                    return;
                }

                let mut doc = toto_yaml::FileEntity::from_url(import_url);
                if let Err(err) = doc.fetch() {
                    add_with_loc(toto_parser::ParseError::Custom(err.to_string()), n, ast);
                    return;
                }

                let doc_handle = ast.add_node(doc.into());
                let doc_root = YamlParser::parse(doc_handle, ast).unwrap();
                let imported_handle = ToscaParser::parse(doc_root, ast).unwrap();

                ast.add_edge(n, imported_handle, crate::Relation::ImportFile.into());
            });

        Some(file_handle)
    }
}
