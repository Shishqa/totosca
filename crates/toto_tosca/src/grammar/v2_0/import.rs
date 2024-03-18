use std::{collections::HashSet, marker::PhantomData};

use toto_ast::RelationParser;
use toto_parser::{add_with_loc, Schema};

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::value::{self};

pub struct Import;
impl<R> toto_ast::Linker<usize, R> for Import
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: usize) -> R = |i| crate::Relation::Import(i).into();
}

pub struct ImportUrl;
impl<R> toto_ast::Linker<(), R> for ImportUrl
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::Url.into();
}

pub struct ImportProfile;
impl<R> toto_ast::Linker<(), R> for ImportProfile
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::ImportProfile.into();
}

pub struct ImportNamespace;
impl<R> toto_ast::Linker<(), R> for ImportNamespace
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::ImportNamespace.into();
}

pub struct ImportRepository;
impl<R> toto_ast::Linker<(), R> for ImportRepository
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::ImportRepository.into();
}

#[derive(Debug)]
pub struct ImportDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> Schema<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "url" => toto_parser::Field::<ImportUrl, value::String>::parse,
        "profile" => toto_parser::Field::<ImportProfile, value::String>::parse,
        "repository" => toto_parser::Field::<ImportRepository, value::String>::parse,
        "namespace" => toto_parser::Field::<ImportNamespace, value::String>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] = &[
        |fields: &HashSet<String>| {
            if fields.contains("url") && fields.contains("profile") {
                Some(toto_parser::ParseError::Custom(
                    "url and profile are mutually exclusive".to_string(),
                ))
            } else {
                None
            }
        },
        |fields: &HashSet<String>| {
            if !fields.contains("url") && !fields.contains("profile") {
                Some(toto_parser::ParseError::MissingField("url or profile"))
            } else {
                None
            }
        },
        |fields: &HashSet<String>| {
            if fields.contains("repository") && !fields.contains("url") {
                Some(toto_parser::ParseError::Custom(
                    "can only be used when a url is specified".to_string(),
                ))
            } else {
                None
            }
        },
    ];
}

impl<E, R, V> toto_ast::EntityParser<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let import = add_with_loc(crate::Entity::Definition, n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| Self::parse_schema(import, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| {
                ast.add_edge(import, n, crate::Relation::Url.into());
                import
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(import)
    }
}
