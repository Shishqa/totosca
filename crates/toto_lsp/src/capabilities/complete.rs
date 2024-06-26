use lsp_types::{CompletionItem, CompletionItemKind};
use petgraph::{visit::EdgeRef, Direction::Incoming};
use toto_parser::get_yaml_len;
use toto_tosca::AsToscaRelation;
use toto_yaml::{AsFileEntity, AsFileRelation, AsYamlEntity};

use crate::models;

pub(crate) fn complete_at(
    ast: &mut toto_ast::AST<models::Entity, models::Relation>,
    uri: &url::Url,
    lineno: u32,
    charno: u32,
) -> Vec<CompletionItem> {
    eprintln!("looking for {}", uri);
    let file_handle = ast
        .node_indices()
        .find(|n| matches!(ast.node_weight(*n).unwrap().as_file(), Some(f) if &f.url == uri))
        .unwrap();

    let params_pos = toto_yaml::from_lc(
        ast.node_weight(file_handle)
            .unwrap()
            .as_file()
            .unwrap()
            .content
            .as_ref()
            .unwrap(),
        lineno,
        charno,
    );

    let lookuper = ast
        .edges_directed(file_handle, Incoming)
        .filter_map(|e| e.weight().as_file().map(|pos| (pos.0, e.source())))
        .filter_map(
            |(pos, source)| match ast.node_weight(source).unwrap().as_yaml() {
                Some(_) => Some((pos, get_yaml_len(source, &ast), source)),
                _ => None,
            },
        )
        .filter_map(|(pos, len, source)| {
            if pos <= params_pos && params_pos <= pos + len {
                Some(ast.edges_directed(source, Incoming))
            } else {
                None
            }
        })
        .flatten()
        .find_map(|e| match e.weight().as_tosca() {
            Some(toto_tosca::Relation::Ref(referencer)) => {
                Some((e.id(), referencer.lookuper.clone()))
            }
            _ => None,
        });

    if lookuper.is_none() {
        eprintln!("can't provide completion (no semantic) {}", params_pos);
        return vec![];
    }
    let lookuper = lookuper.unwrap();

    lookuper
        .1
        .lookup_suggests(&ast, lookuper.0)
        .into_iter()
        .map(|(name, detail, rel)| {
            let mut item = lsp_types::CompletionItem::new_simple(name, detail.unwrap_or_default());

            item.kind = match rel {
                toto_tosca::Relation::Type(_) => Some(CompletionItemKind::TYPE_PARAMETER),
                toto_tosca::Relation::Definition(_) => Some(CompletionItemKind::REFERENCE),
                _ => None,
            };

            item
        })
        .collect::<Vec<_>>()
}
