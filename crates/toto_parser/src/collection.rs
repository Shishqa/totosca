use std::marker::PhantomData;

use crate::{add_with_loc, ParseCompatibleEntity, ParseCompatibleRelation, ParseError};

pub struct Collection<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> toto_ast::RelationParser<E, R> for Collection<K, V>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
    K: toto_ast::Linker<std::string::String, R>,
    V: toto_ast::EntityParser<E, R>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(items) = toto_yaml::as_map(n, ast).or_else(|| {
            add_with_loc(ParseError::UnexpectedType("map"), n, ast);
            None
        }) {
            items.for_each(|(k, v)| {
                if let Some((k_str, v_handle)) = toto_yaml::as_string(k, ast)
                    .or_else(|| {
                        add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                        None
                    })
                    .zip(V::parse(v, ast))
                {
                    ast.add_edge(root, v_handle, K::L(k_str));
                }
            });
        }
    }
}
