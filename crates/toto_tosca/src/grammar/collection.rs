use std::marker::PhantomData;

use crate::{add_with_loc, ParseError, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::field::Field;

pub struct CollectionRelator<C>(PhantomData<C>);

impl<C, E, R> toto_parser::RelationParser<E, R> for CollectionRelator<C>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    C: toto_parser::ValueRelationParser<E, R, String>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(items) = toto_yaml::as_map(n, ast).or_else(|| {
            add_with_loc(ParseError::UnexpectedType("map"), n, ast);
            None
        }) {
            items.for_each(|(k, v)| {
                if let Some(k_str) = toto_yaml::as_string(k, ast).cloned().or_else(|| {
                    add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                    None
                }) {
                    C::parse(k_str.0.clone(), root, v, ast);
                }
            });
        }
    }
}

pub type Collection<K, V> = CollectionRelator<Field<K, V>>;
