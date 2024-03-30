use std::collections::HashMap;

use petgraph::visit::EdgeRef;
use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

type Namespace = HashMap<
    toto_ast::GraphHandle,
    HashMap<(crate::Relation, crate::Entity), toto_ast::GraphHandle>,
>;

pub struct Lookup {
    ns: Namespace,
}

impl Lookup {
    pub fn from_ast<E, R>(ast: &toto_ast::AST<E, R>) -> Self
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let mut ns = Namespace::new();

        ast.edge_references()
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Type(type_name)) => Some((
                    e.source(),
                    (
                        crate::Relation::Type(type_name.clone()),
                        *ast.node_weight(e.target()).unwrap().as_tosca().unwrap(),
                    ),
                    e.target(),
                )),
                _ => None,
            })
            .for_each(|(n, rel, t)| {
                if let Some(n_ns) = ns.get_mut(&n) {
                    n_ns.insert(rel, t);
                } else {
                    let mut n_ns =
                        HashMap::<(crate::Relation, crate::Entity), toto_ast::GraphHandle>::new();
                    n_ns.insert(rel, t);
                    ns.insert(n, n_ns);
                }
            });

        Self { ns }
    }

    pub fn lookup<E, R>(&self, ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.edge_references()
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::RefType) => Some((
                    e.source(),
                    (
                        crate::Relation::Type(toto_yaml::as_string(e.target(), ast).unwrap()),
                        *ast.node_weight(e.source()).unwrap().as_tosca().unwrap(),
                    ),
                    crate::Relation::HasType,
                    e.target(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(n, rel_ref, new_rel, err_target)| {
                let root = ast
                    .edges_directed(n, petgraph::Direction::Incoming)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::RefRoot) => Some(e.source()),
                        _ => None,
                    })
                    .unwrap();

                if let Some(target_type) = self.ns.get(&root).and_then(|n_ns| n_ns.get(&rel_ref)) {
                    if ast.edges_connecting(n, *target_type).count() > 0 {
                        return;
                    }

                    ast.add_edge(n, *target_type, new_rel.into());
                } else {
                    add_with_loc(
                        toto_parser::ParseError::Custom("unknown type".to_string()),
                        err_target,
                        ast,
                    );
                }
            });
    }
}
