use lsp_types::Location;
use petgraph::{
    visit::EdgeRef,
    Direction::{Incoming, Outgoing},
};
use toto_parser::{get_yaml_len, AsParseLoc};
use toto_tosca::{AsToscaRelation, ImportTargetRelation};
use toto_yaml::{AsFileEntity, AsFileRelation, AsYamlEntity};

use crate::models;

pub fn goto_definition(
    ast: &mut toto_ast::AST<models::Entity, models::Relation>,
    uri: &url::Url,
    lineno: u32,
    charno: u32,
) -> Option<Location> {
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

    let Some((semantic_token, semantic_rel)) = ast
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
                Some((e.source(), referencer.lookuper.then.clone()))
            }
            Some(toto_tosca::Relation::ImportUrl(_)) => {
                Some((e.source(), toto_tosca::Relation::from(ImportTargetRelation)))
            }
            _ => None,
        })
    else {
        eprintln!("can't go to definition (no semantic) {}", params_pos);
        return None;
    };

    let Some(goto_target) = ast.edges_directed(semantic_token, Outgoing).find_map(|e| {
        if e.weight().as_tosca() == Some(&semantic_rel) {
            Some(e.target())
        } else {
            None
        }
    }) else {
        eprintln!("can't go to definition (no target)");
        return None;
    };

    let (target_file, target_pos) =
        if let Some(target_file) = ast.node_weight(goto_target).unwrap().as_file() {
            (target_file, 0)
        } else {
            ast.edges_directed(goto_target, Outgoing)
                .filter_map(|e| match e.weight().as_parse_loc() {
                    Some(_) => Some(ast.edges_directed(e.target(), Outgoing)),
                    _ => None,
                })
                .flatten()
                .find_map(|e| e.weight().as_file().map(|loc| (e.target(), loc.0)))
                .map(|(file_handle, pos)| {
                    let file = ast.node_weight(file_handle).unwrap().as_file().unwrap();
                    (file, pos)
                })
                .unwrap()
        };

    if target_file.url.scheme() == "builtin" {
        eprintln!("can't go to builtin spec");
        return None;
    }

    let (target_l, target_c) = toto_yaml::get_lc(target_file.content.as_ref().unwrap(), target_pos);

    Some(Location::new(
        target_file.url.clone(),
        lsp_types::Range::new(
            lsp_types::Position::new(target_l, target_c),
            lsp_types::Position::new(target_l, target_c + 1),
        ),
    ))
}
