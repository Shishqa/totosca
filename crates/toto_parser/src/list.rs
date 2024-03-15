use std::marker::PhantomData;

use crate::{
    add_with_loc, EntityParser, Linker, ParseCompatibleEntity, ParseCompatibleRelation, ParseError,
    RelationParser,
};

pub struct List<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> RelationParser<E, R> for List<K, V>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
    K: Linker<usize, R>,
    V: EntityParser<E, R>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        toto_yaml::as_list(n, ast)
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("list"), n, ast);
                None
            })
            .and_then(|items| {
                items.for_each(|(i, v)| {
                    V::parse(v, ast).and_then(|v_handle| {
                        ast.add_edge(root, v_handle, K::L(i));
                        Some(v_handle)
                    });
                });
                Some(n)
            });
    }
}