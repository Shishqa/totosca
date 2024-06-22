use std::collections::HashMap;

use petgraph::{
    algo::toposort,
    data::DataMap,
    visit::{EdgeFiltered, EdgeRef, NodeFiltered, NodeRef},
    Direction::Outgoing,
};
use toto_parser::{add_with_loc, ParseError};

use crate::{DefinitionRelation, ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Derive;

impl Derive {
    pub fn inherit_all_definitions<E, R>(ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let _ = Self::topo_iter_definitions(ast)
            .map_err(|e| {
                add_with_loc(
                    ParseError::Custom("circular dependency detected".to_string()),
                    e,
                    ast,
                );
            })
            .map(|definitions| {
                definitions.for_each(|def_handle| {
                    Self::inherit(def_handle, ast);
                });
            });
    }

    fn topo_iter_definitions<E, R>(
        ast: &toto_ast::AST<E, R>,
    ) -> Result<impl Iterator<Item = toto_ast::GraphHandle>, toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let def_graph = EdgeFiltered::from_fn(ast, |e| {
            matches!(
                e.weight().as_tosca(),
                Some(
                    crate::Relation::HasType(_)
                        | crate::Relation::DerivedFrom(_)
                        | crate::Relation::Definition(_)
                        | crate::Relation::Assignment(_)
                )
            )
        });
        let def_graph = NodeFiltered::from_fn(&def_graph, |n| {
            ast.node_weight(n.id()).unwrap().as_tosca().is_some()
        });

        toposort(&def_graph, None)
            .map_err(|err| err.node_id())
            .map(|v| v.into_iter().rev())
    }

    fn inherit<E, R>(def_handle: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let Some((_inherit_kind, parent_handle)) = ast
            .edges_directed(def_handle, Outgoing)
            .find_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::HasType(_) | crate::Relation::DerivedFrom(_)) => {
                    Some((e.weight().as_tosca().unwrap().clone(), e.target()))
                }
                _ => None,
            })
        else {
            return;
        };

        let parent_definitions = ast
            .edges_directed(parent_handle, Outgoing)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Definition(_)) => Some((
                    (
                        e.weight().as_tosca().unwrap().clone(),
                        *ast.node_weight(e.target()).unwrap().as_tosca().unwrap(),
                    ),
                    e.target(),
                )),
                _ => None,
            })
            .collect::<HashMap<(crate::Relation, crate::Entity), toto_ast::GraphHandle>>();

        let child_definitions = ast
            .edges_directed(def_handle, Outgoing)
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Definition(_) | crate::Relation::Assignment(_))
                    if ast.node_weight(e.target()).unwrap().as_tosca().is_some() =>
                {
                    Some((
                        (
                            e.weight().as_tosca().unwrap().clone(),
                            *ast.node_weight(e.target()).unwrap().as_tosca().unwrap(),
                        ),
                        e.target(),
                    ))
                }
                _ => None,
            })
            .collect::<HashMap<(crate::Relation, crate::Entity), toto_ast::GraphHandle>>();

        child_definitions
            .iter()
            .for_each(|((rel, ent), child_def_handle)| {
                match rel {
                    crate::Relation::Definition(_) => {
                        let Some(refined_def) = parent_definitions.get(&(rel.clone(), *ent)) else {
                            return;
                        };
                        // TODO: check refinement
                        ast.add_edge(
                            *child_def_handle,
                            *refined_def,
                            crate::Relation::from(crate::RefinedFromRelation).into(),
                        );
                    }
                    crate::Relation::Assignment(crate::AssignmentRelation(name)) => {
                        let Some(assigned_def) = parent_definitions.get(&(
                            crate::Relation::from(DefinitionRelation(name.clone())),
                            *ent,
                        )) else {
                            add_with_loc(
                                toto_parser::ParseError::Custom(format!("unknown {:?}", ent)),
                                *child_def_handle,
                                ast,
                            );
                            return;
                        };

                        ast.add_edge(
                            *child_def_handle,
                            *assigned_def,
                            crate::Relation::from(crate::DefinedByRelation).into(),
                        );
                    }
                    _ => {}
                }
            });

        parent_definitions
            .iter()
            .for_each(|((rel, ent), parent_def_handle)| match rel {
                crate::Relation::Definition(_) => {
                    let None = child_definitions.get(&(rel.clone(), *ent)) else {
                        return;
                    };
                    ast.add_edge(def_handle, *parent_def_handle, rel.clone().into());
                }
                _ => {}
            });
    }
}
