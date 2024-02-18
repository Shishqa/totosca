use std::marker::PhantomData;

use crate::{
    parse::{add_error, ParseError, ParseLoc},
    tosca::ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
};

pub struct List<E, R, P>(
    pub toto_ast::GraphHandle,
    pub toto_ast::GraphHandle,
    pub PhantomData<(E, R, P)>,
)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;

impl<E, R, P> toto_ast::Parse<E, R> for List<E, R, P>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        if let toto_yaml::Entity::List = t {
            toto_yaml::iter_items(self.0, ast)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|(order, v)| {
                    let value = P::from(*v).parse(ast);
                    ast.add_edge(
                        self.1,
                        value,
                        toto_tosca::Relation::OrderedSubdef(*order).into(),
                    );
                });
        } else {
            let e = ast.add_node(ParseError::UnexpectedType("list").into());
            ast.add_edge(e, self.0, ParseLoc.into());
        }

        self.1
    }
}

pub struct KeyedList<E, R, P>(
    pub toto_ast::GraphHandle,
    pub toto_ast::GraphHandle,
    pub PhantomData<(E, R, P)>,
)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;

impl<E, R, P> toto_ast::Parse<E, R> for KeyedList<E, R, P>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        if let toto_yaml::Entity::List = t {
            toto_yaml::iter_items(self.0, ast)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|(order, item)| {
                    let i = ast.node_weight(item).unwrap();
                    let i = i.as_yaml().unwrap();

                    if let toto_yaml::Entity::Map = i {
                        let keys = toto_yaml::iter_keys(self.0, ast).collect::<Vec<_>>();
                        if keys.len() != 1 {
                            add_error(
                                item,
                                ast,
                                ParseError::Custom("should have only one key".to_string()),
                            );
                            return;
                        }

                        let (k, v) = keys.iter().next().unwrap();
                        let key = ast
                            .node_weight(*k)
                            .unwrap()
                            .as_yaml()
                            .map(|key| match key {
                                toto_yaml::Entity::Str(str_key) => Some(str_key.to_string()),
                                _ => None,
                            })
                            .unwrap();

                        if let None = key {
                            add_error(*k, ast, ParseError::UnexpectedType("string"));
                            return;
                        }

                        let value = P::from(*v).parse(ast);
                        ast.add_edge(
                            self.1,
                            value,
                            toto_tosca::Relation::NamedSubdef(key.unwrap()).into(),
                        );
                        ast.add_edge(
                            self.1,
                            value,
                            toto_tosca::Relation::OrderedSubdef(*order).into(),
                        );
                    } else {
                        let e = ast.add_node(ParseError::UnexpectedType("map").into());
                        ast.add_edge(e, item, ParseLoc.into());
                    }
                });
        } else {
            let e = ast.add_node(ParseError::UnexpectedType("list").into());
            ast.add_edge(e, self.0, ParseLoc.into());
        }

        self.1
    }
}
