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

        let target_str = toto_yaml::as_string(target, ast)
            .map(|v| v.0.clone())
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("string"),
                    target,
                    ast,
                );
                None
            });
        if target_str.is_none() {
            return;
        }
        let target_str = target_str.unwrap();
        let target_rel = (self.what_rel)(target_str);

        let mut path = vec![source];
        let mut curr_node = source;
        let root = loop {
            let root = ast
                .edges_directed(curr_node, petgraph::Direction::Outgoing)
                .find_map(|e| {
                    if e.weight().as_tosca() == Some(&self.root.0) {
                        Some(e.target())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("expected {:?} to have {:?} entity", path, self.root.1));

            if ast.node_weight(root).unwrap().as_tosca() == Some(&self.root.1) {
                break root;
            }
            curr_node = root;
            path.push(curr_node);
        };

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
                .any(|e| matches!(e.weight().as_tosca(), Some(rel) if *rel == self.then))
            {
                return;
            }

            ast.add_edge(source, lookuped, self.then.clone().into());
        } else {
            add_with_loc(
                toto_parser::ParseError::Custom(format!(
                    "unknown {:?} {:?}",
                    self.what, target_rel
                )),
                target,
                ast,
            );
        }
    }

    pub fn lookup_suggests<E, R>(
        &self,
        ast: &toto_ast::AST<E, R>,
        e: toto_ast::EdgeHandle,
    ) -> Vec<(String, Option<String>, crate::Relation)>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let (source, _) = ast.edge_endpoints(e).unwrap();

        let mut path = vec![source];
        let mut curr_node = source;
        let root = loop {
            let root = ast
                .edges_directed(curr_node, petgraph::Direction::Outgoing)
                .find_map(|e| {
                    if e.weight().as_tosca() == Some(&self.root.0) {
                        Some(e.target())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("expected {:?} to have {:?} entity", path, self.root.1));

            if ast.node_weight(root).unwrap().as_tosca() == Some(&self.root.1) {
                break root;
            }
            curr_node = root;
            path.push(curr_node);
        };

        let target_rel = (self.what_rel)(String::new());
        let lookuped = ast
            .edges_directed(root, petgraph::Direction::Outgoing)
            .filter_map(|e| {
                if let Some(rel) = e.weight().as_tosca() {
                    if e.target() != source
                        && std::mem::discriminant(rel) == std::mem::discriminant(&target_rel)
                        && ast.node_weight(e.target()).unwrap().as_tosca() == Some(&self.what)
                    {
                        let name = match rel {
                            crate::Relation::Type(type_rel) => type_rel.0.clone(),
                            crate::Relation::Definition(def_rel) => def_rel.0.clone(),
                            _ => String::new(),
                        };
                        return Some((name, rel, e.target()));
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        lookuped
            .into_iter()
            .map(|(name, rel, n)| {
                let description = ast
                    .edges_directed(n, petgraph::Direction::Outgoing)
                    .find_map(|e| match e.weight().as_tosca() {
                        Some(crate::Relation::Description(_)) => Some(
                            toto_yaml::as_string(e.target(), ast)
                                .expect("expected string")
                                .0
                                .clone(),
                        ),
                        _ => None,
                    });

                (name, description, rel.clone())
            })
            .collect()
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
