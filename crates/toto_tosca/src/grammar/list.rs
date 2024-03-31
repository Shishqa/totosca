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
                        v_handle
                    });
                });
                n
            });
    }
}
