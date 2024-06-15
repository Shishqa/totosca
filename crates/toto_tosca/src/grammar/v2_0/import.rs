use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, RelationParser};

use crate::{
    grammar::{field::Field, ToscaDefinitionsVersion},
    ImportNamespaceRelation, ImportProfileRelation, ImportRepositoryRelation, ImportUrlRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};

use super::value::{self};

#[derive(Debug)]
pub struct ImportDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ImportEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "url" => Field::<ImportUrlRelation, value::StringValue>::parse,

        "profile" => Field::<ImportProfileRelation, value::StringValue>::parse,
        "repository" => Field::<ImportRepositoryRelation, value::StringValue>::parse,
        "namespace" => Field::<ImportNamespaceRelation, value::StringValue>::parse,
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

impl<E, R, V> toto_parser::EntityParser<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let import = add_with_loc(crate::Entity::from(crate::ImportEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(import, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    import,
                    n,
                    crate::Relation::from(crate::ImportUrlRelation).into(),
                );
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
