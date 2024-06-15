use std::marker::PhantomData;

use crate::{add_with_loc, ParseError, ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Collection<K, V>(PhantomData<(K, V)>);

impl<K, V, E, R> toto_parser::RelationParser<E, R> for Collection<K, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    K: From<String>,
    V: toto_parser::EntityParser<E, R>,
    crate::Relation: From<K>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(items) = toto_yaml::as_map(n, ast).or_else(|| {
            add_with_loc(ParseError::UnexpectedType("map"), n, ast);
            None
        }) {
            items.for_each(|(k, v)| {
                let k_str = toto_yaml::as_string(k, ast).cloned().or_else(|| {
                    add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                    None
                });

                k_str.zip(V::parse(v, ast)).inspect(|(k_str, v_handle)| {
                    ast.add_edge(
                        root,
                        *v_handle,
                        crate::Relation::from(K::from(k_str.0.clone())).into(),
                    );
                    ast.add_edge(
                        *v_handle,
                        root,
                        crate::Relation::Root(crate::RootRelation).into(),
                    );
                });
            });
        }
    }
}
