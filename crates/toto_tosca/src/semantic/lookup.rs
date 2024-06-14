use petgraph::{data::DataMap, visit::EdgeRef};
use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SimpleLookuper {
    pub root: (crate::Relation, crate::Entity),
    pub what: crate::Entity,
    pub what_rel: fn(String) -> crate::Relation,
    pub then: crate::Relation,
}

impl SimpleLookuper {
    pub fn lookup<E, R>(&self, ast: &mut toto_ast::AST<E, R>, e: toto_ast::EdgeHandle)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let (source, target) = ast.edge_endpoints(e).unwrap();

        let target_str = toto_yaml::as_string(target, ast).expect("expected string");
        let target_rel = (self.what_rel)(target_str.0.clone());

        let root = ast
            .edges_directed(source, petgraph::Direction::Incoming)
            .find_map(|e| {
                if e.weight().as_tosca() == Some(&self.root.0)
                    && ast.node_weight(e.source()).unwrap().as_tosca() == Some(&self.root.1)
                {
                    Some(e.source())
                } else {
                    None
                }
            })
            .unwrap();

        let lookuped = ast
            .edges_directed(root, petgraph::Direction::Outgoing)
            .find_map(|e| {
                if e.weight().as_tosca() == Some(&target_rel)
                    && ast.node_weight(e.target()).unwrap().as_tosca() == Some(&self.what)
                {
                    Some(e.target())
                } else {
                    None
                }
            });

        if let Some(lookuped) = lookuped {
            if ast
                .edges_connecting(source, lookuped)
                .find(|e| matches!(e.weight().as_tosca(), Some(rel) if *rel == self.then))
                .is_some()
            {
                return;
            }

            ast.add_edge(source, lookuped, self.then.clone().into());
        } else {
            add_with_loc(
                toto_parser::ParseError::Custom("unknown type".to_string()),
                target,
                ast,
            );
        }
    }
}

pub struct Lookup;

impl Lookup {
    pub fn lookup<E, R>(ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.edge_references()
            .filter_map(|e| match e.weight().as_tosca() {
                Some(crate::Relation::Ref(referencer)) => {
                    Some((e.id(), referencer.lookuper.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(e, lookuper)| {
                lookuper.lookup(ast, e);
            });
    }
}
