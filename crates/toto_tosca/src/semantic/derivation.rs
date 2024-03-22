use std::collections::HashMap;

use petgraph::{
    algo::{toposort, DfsSpace},
    dot::Dot,
    visit::{EdgeFiltered, EdgeRef, IntoNeighbors, NodeFiltered, NodeRef},
    Direction,
};
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

        node_types
            .values()
            .flat_map(|n| {
                ast.edges_directed(*n, Direction::Outgoing)
                    .map(|e| (*n, e.id()))
            })
            .filter_map(|(n, e)| match ast.edge_weight(e).unwrap().as_tosca() {
                Some(crate::Relation::RefType) => Some((n, ast.edge_endpoints(e).unwrap().1)),
                _ => None,
            })
            .filter_map(|(n, type_n)| {
                toto_yaml::as_string(type_n, ast).map(|type_str| (n, type_n, type_str))
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(n, type_n, type_str)| {
                node_types
                    .get(type_str)
                    .map(|target_type| {
                        ast.add_edge(*n, *target_type, crate::Relation::DerivedFrom.into())
                    })
                    .or_else(|| {
                        add_with_loc(
                            ParseError::Custom(format!("type {} not found", type_str)),
                            *type_n,
                            ast,
                        );
                        None
                    });
            });

        ast.edges_directed(file_handle, Direction::Outgoing)
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
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(n, type_n, type_str)| {
                node_types
                    .get(type_str)
                    .or_else(|| {
                        add_with_loc(
                            ParseError::Custom(format!("type {} not found", type_str)),
                            *type_n,
                            ast,
                        );
                        None
                    })
                    .map(|target_type| {
                        ast.add_edge(*n, *target_type, crate::Relation::HasType.into())
                    });
            });

        // let type_graph = EdgeFiltered::from_fn(&*ast, |e| {
        //     matches!(
        //         e.weight().as_tosca(),
        //         Some(crate::Relation::DerivedFrom) | Some(crate::Relation::HasType)
        //     )
        // });
        // let type_graph = NodeFiltered::from_fn(&type_graph, |n| {
        //     matches!(
        //         ast.node_weight(n.id()).unwrap().as_tosca(),
        //         Some(crate::Entity::Definition)
        //     ) && type_graph.neighbors(n).count() > 0
        // });

        // toposort(&type_graph, None);

        // dbg!(&topo);

        // dbg!(Dot::new(&type_graph));

        Some(file_handle)
    }
}
