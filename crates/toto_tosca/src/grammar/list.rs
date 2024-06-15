use std::marker::PhantomData;

use toto_parser::{add_with_loc, ParseError};

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct List<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> toto_parser::RelationParser<E, R> for List<K, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    K: From<usize>,
    V: toto_parser::EntityParser<E, R>,
    crate::Relation: From<K>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        toto_yaml::as_list(n, ast)
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("list"), n, ast);
                None
            })
            .map(|items| {
                items.for_each(|(i, v)| {
                    V::parse(v, ast).map(|v_handle| {
                        ast.add_edge(root, v_handle, crate::Relation::from(K::from(i)).into());
                        ast.add_edge(
                            v_handle,
                            root,
                            crate::Relation::Root(crate::RootRelation).into(),
                        );
                        v_handle
                    });
                });
                n
            });
    }
}

pub struct KeyedList<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> toto_parser::RelationParser<E, R> for KeyedList<K, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    K: From<(String, usize)>,
    V: toto_parser::EntityParser<E, R>,
    crate::Relation: From<K>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        toto_yaml::as_list(n, ast)
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("list"), n, ast);
                None
            })
            .map(|items| {
                items.for_each(|(i, v)| {
                    if let Some(items) = toto_yaml::as_map(v, ast).or_else(|| {
                        add_with_loc(ParseError::UnexpectedType("map"), n, ast);
                        None
                    }) {
                        let mut items = items.take(2);

                        if let Some((k, v)) = items.next() {
                            let k_str = toto_yaml::as_string(k, ast).cloned().or_else(|| {
                                add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                                None
                            });

                            k_str.zip(V::parse(v, ast)).inspect(|(k_str, v_handle)| {
                                ast.add_edge(
                                    root,
                                    *v_handle,
                                    crate::Relation::from(K::from((k_str.0.clone(), i))).into(),
                                );
                                ast.add_edge(
                                    *v_handle,
                                    root,
                                    crate::Relation::Root(crate::RootRelation).into(),
                                );
                            });
                        } else {
                            add_with_loc(ParseError::Custom("expected a key".to_string()), v, ast);
                            return;
                        }

                        if items.next().is_some() {
                            add_with_loc(
                                ParseError::Custom("expected only one key".to_string()),
                                v,
                                ast,
                            );
                        }
                    }
                });
                n
            });
    }
}
