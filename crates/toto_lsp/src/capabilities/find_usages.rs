use lsp_types::Location;
use petgraph::{
    visit::EdgeRef,
    Direction::{Incoming, Outgoing},
};
use toto_parser::{get_yaml_len, AsParseLoc};
use toto_tosca::{AsToscaEntity, AsToscaRelation};
use toto_yaml::{AsFileEntity, AsFileRelation, AsYamlEntity, AsYamlRelation};

use crate::models;

pub fn find_usages(
    ast: &mut toto_ast::AST<models::Entity, models::Relation>,
    uri: &url::Url,
    lineno: u32,
    charno: u32,
) -> Vec<Location> {
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

    let Some(semantic_token) = ast
        .edges_directed(file_handle, Incoming)
        .filter_map(|e| e.weight().as_file().map(|pos| (pos.0, e.source())))
        .filter_map(
            |(pos, source)| match ast.node_weight(source).unwrap().as_yaml() {
                Some(_) => Some((pos, get_yaml_len(source, ast), source)),
                _ => None,
            },
        )
        .filter_map(|(pos, len, source)| {
            let new_source = ast
                .edges_directed(source, Outgoing)
                .chain(ast.edges_directed(source, Incoming))
                .find_map(|e| match e.weight().as_yaml() {
                    Some(toto_yaml::Relation::MapValue(_)) if e.source() == source => {
                        Some(e.target())
                    }
                    _ => None,
                })?;

            if pos <= params_pos && params_pos <= pos + len {
                Some(ast.edges_directed(new_source, Incoming))
            } else {
                None
            }
        })
        .flatten()
        .find_map(|e| {
            if e.weight().as_parse_loc().is_some() && ast[e.source()].as_tosca().is_some() {
                Some(e.source())
            } else {
                None
            }
        })
    else {
        eprintln!("can't find usages (no semantic) {}", params_pos);
        return vec![];
    };

    let usage_refs = ast
        .edges_directed(semantic_token, Incoming)
        .filter_map(|e| match e.weight().as_tosca() {
            Some(
                toto_tosca::Relation::HasType(_)
                | toto_tosca::Relation::DerivedFrom(_)
                | toto_tosca::Relation::TargetNode(_)
                | toto_tosca::Relation::ValidSourceNodeType(_)
                | toto_tosca::Relation::ValidTargetNodeType(_)
                | toto_tosca::Relation::ValidCapabilityType(_)
                | toto_tosca::Relation::ValidRelationshipType(_),
            ) => Some(e.source()),
            _ => None,
        })
        .collect::<Vec<_>>();

    if usage_refs.is_empty() {
        eprintln!("no usages");
        return vec![];
    };

    usage_refs
        .into_iter()
        .filter_map(|n| {
            ast.edges_directed(n, Outgoing)
                .filter_map(|e| match e.weight().as_parse_loc() {
                    Some(_) => Some(ast.edges_directed(e.target(), Outgoing)),
                    _ => None,
                })
                .flatten()
                .find_map(|e| e.weight().as_file().map(|loc| (e.target(), loc.0)))
                .and_then(|(file_handle, pos)| {
                    let file = ast.node_weight(file_handle).unwrap().as_file().unwrap();

                    if file.url.scheme() == "builtin" {
                        eprintln!("can't reference builtin");
                        return None;
                    }

                    let (lineno, charno) = toto_yaml::get_lc(file.content.as_ref().unwrap(), pos);
                    Some(Location::new(
                        file.url.clone(),
                        lsp_types::Range::new(
                            lsp_types::Position::new(lineno, charno),
                            lsp_types::Position::new(lineno, charno + 1),
                        ),
                    ))
                })
        })
        .collect::<Vec<_>>()
}
