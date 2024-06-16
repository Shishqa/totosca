use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, RelationParser};

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        field_ref::{DefRef, TypeRef},
        list::ListRelator,
        v2_0, ToscaDefinitionsVersion,
    },
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefinitionRelation,
    DescriptionRelation, HasFileRelation, PrimaryArtifactRelation, RepositoryRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, VersionRelation,
};

#[derive(Debug)]
pub struct ArtifactDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ImplementationDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ArtifactDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ArtifactEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::ArtifactEntity, crate::HasTypeRelation>::parse,
        "file" => Field::<HasFileRelation, v2_0::value::StringValue>::parse,
        "repository" => Field::<RepositoryRelation, v2_0::value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, v2_0::value::StringValue>::parse,
        "deploy_path" => Field::<DefinitionRelation, v2_0::value::StringValue>::parse,
        "artifact_version" => Field::<VersionRelation, v2_0::value::StringValue>::parse,
        "checksum" => Field::<ChecksumRelation, v2_0::value::StringValue>::parse,
        "checksum_algorithm" => Field::<ChecksumAlgorithmRelation, v2_0::value::StringValue>::parse,
        "properties" => Collection::<AssignmentRelation, v2_0::value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] = &[
        |fields: &HashSet<String>| mandatory(fields, "type"),
        |fields: &HashSet<String>| mandatory(fields, "file"),
    ];
}

impl<E, R, V> toto_parser::Schema<E, R> for ImplementationDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ImplementationEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "primary" => v2_0::ArtifactRefOrDefinition::<V, PrimaryArtifactRelation>::parse,
        "dependencies" => ListRelator::<v2_0::ArtifactRefOrDefinition<V, PrimaryArtifactRelation>>::parse,
        "timeout" => Field::<DefinitionRelation, v2_0::value::IntValue>::parse,
        "operation_host" => Field::<DefinitionRelation, v2_0::value::StringValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ImplementationDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let implementation = add_with_loc(crate::Entity::from(crate::ImplementationEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| {
                <Self as toto_parser::Schema<E, R>>::parse_schema(implementation, items, ast)
            })
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                DefRef::<
                        crate::NodeEntity,
                        crate::ArtifactEntity,
                        crate::PrimaryArtifactRelation,
                    >::parse(implementation, n, ast);
                implementation
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(implementation)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ArtifactDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}
