use std::marker::PhantomData;

use crate::{
    add_with_loc, EntityParser, Linker, ParseCompatibleEntity, ParseCompatibleRelation, ParseError,
    RelationParser,
};

pub struct Collection<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> RelationParser<E, R> for Collection<K, V>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
    K: Linker<std::string::String, R>,
    V: EntityParser<E, R>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        toto_yaml::as_map(n, ast)
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("map"), n, ast);
                None
            })
            .and_then(|items| {
                items.for_each(|(k, v)| {
                    toto_yaml::as_string(k, ast)
                        .or_else(|| {
                            add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                            None
                        })
                        .zip(V::parse(v, ast))
                        .and_then(|(k_str, v_handle)| {
                            ast.add_edge(root, v_handle, K::L(k_str));
                            Some(v_handle)
                        });
                });
                Some(n)
            });
    }
}
