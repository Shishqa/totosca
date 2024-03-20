use std::collections::HashMap;

use petgraph::{visit::EdgeRef, Direction};
use toto_parser::{add_with_loc, ParseError};

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct TypeResolver;

impl<E, R> toto_parser::EntityParser<E, R> for TypeResolver
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let node_types = ast
            .edges(file_handle)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::NodeType(type_name)) => Some((type_name.clone(), e.target())),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        dbg!(&node_types);

        let nodes = ast
            .edges_directed(file_handle, Direction::Outgoing)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::ServiceTemplate) => {
                    Some(ast.edges_directed(e.target(), Direction::Outgoing))
                }
                _ => None,
            })
            .flatten()
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::NodeTemplate(_)) => Some(e.target()),
                _ => None,
            })
            .filter_map(|n| {
                ast.edges_directed(n, Direction::Outgoing)
                    .filter_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::RefType) => Some((n, e.target())),
                        _ => None,
                    })
                    .find_map(|(n, type_n)| {
                        toto_yaml::as_string(type_n, ast)
                            .map(|type_str| (n, type_n, type_str.clone()))
                    })
            })
            .collect::<Vec<_>>();

        dbg!(&nodes);

        nodes.iter().for_each(|(n, type_n, type_str)| {
            let target_type = node_types.get(type_str).or_else(|| {
                add_with_loc(
                    ParseError::Custom(format!("type {} not found", type_str)),
                    *type_n,
                    ast,
                );
                None
            });
            if target_type.is_none() {
                return;
            }
            let target_type = target_type.unwrap();

            ast.add_edge(*n, *target_type, crate::Relation::HasType.into());
        });

        Some(file_handle)
    }
}
