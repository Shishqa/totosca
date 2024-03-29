use std::collections::HashMap;

use petgraph::visit::EdgeRef;
use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Lookup;

type Namespace = HashMap<
    toto_ast::GraphHandle,
    HashMap<(crate::Relation, crate::Entity), toto_ast::GraphHandle>,
>;

impl Lookup {
    fn collect_namespace<E, R>(ast: &mut toto_ast::AST<E, R>) -> Namespace
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let mut res = Namespace::new();

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
                if let Some(n_ns) = res.get_mut(&n) {
                    n_ns.insert(rel, t);
                } else {
                    let mut n_ns =
                        HashMap::<(crate::Relation, crate::Entity), toto_ast::GraphHandle>::new();
                    n_ns.insert(rel, t);
                    res.insert(n, n_ns);
                }
            });

        res
    }

    pub fn lookup<E, R>(ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let ns = Self::collect_namespace(ast);

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

                let n_ns = ns.get(&root).unwrap(); // TODO: handle no types in root
                if let Some(target_type) = n_ns.get(&rel_ref) {
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
