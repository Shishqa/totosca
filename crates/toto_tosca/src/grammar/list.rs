use std::marker::PhantomData;

use toto_parser::{add_with_loc, ParseError};

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::field::Field;

pub struct ListRelator<C>(PhantomData<C>);

impl<C, E, R> toto_parser::RelationParser<E, R> for ListRelator<C>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    C: toto_parser::ValueRelationParser<E, R, usize>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(items) = toto_yaml::as_list(n, ast).or_else(|| {
            add_with_loc(ParseError::UnexpectedType("list"), n, ast);
            None
        }) {
            items.for_each(|(i, v)| {
                C::parse(i, root, v, ast);
            });
        }
    }
}

pub type List<C, V> = ListRelator<Field<C, V>>;

pub struct KeyedListRelator<C>(PhantomData<C>);

impl<C, E, R> toto_parser::RelationParser<E, R> for KeyedListRelator<C>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    C: toto_parser::ValueRelationParser<E, R, (String, usize)>,
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
                            if let Some(k_str) =
                                toto_yaml::as_string(k, ast).cloned().or_else(|| {
                                    add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                                    None
                                })
                            {
                                C::parse((k_str.0.clone(), i), root, v, ast);
                            }
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

pub type KeyedList<C, V> = KeyedListRelator<Field<C, V>>;
