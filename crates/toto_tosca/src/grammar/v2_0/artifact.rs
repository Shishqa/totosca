use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{
        collection::Collection, field::Field, field_ref::FieldRef, list::List,
        ToscaDefinitionsVersion,
    },
    semantic::SimpleLookuper,
    ArtifactEntity, AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation,
    DefaultRelation, DefinitionRelation, DependencyArtifactRelation, DerivedFromRelation,
    DescriptionRelation, EntrySchemaRelation, ExternalSchemaRelation, FileEntity, FileExtRelation,
    HasTypeRelation, KeySchemaRelation, MappingRelation, MetadataRelation, MimeTypeRelation,
    PrimaryArtifactRelation, RefDerivedFromRelation, RefHasFileRelation, RefHasTypeRelation,
    RepositoryRelation, RequiredRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    TypeRelation, ValidationRelation, ValueRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct ArtifactTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ArtifactDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ImplementationDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ArtifactRefOrDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ArtifactTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ArtifactEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => |r, n, ast| FieldRef::derived_from(crate::Entity::from(ArtifactEntity)).parse(r, n, ast),
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "mime_type" => Field::<MimeTypeRelation, value::StringValue>::parse,
        "file_ext" => List::<FileExtRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for ArtifactDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ArtifactEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => |r, n, ast| FieldRef::has_type(crate::Entity::from(ArtifactEntity)).parse(r, n, ast),
        "file" => Field::<RefHasFileRelation, value::StringValue>::parse,
        "repository" => Field::<RepositoryRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "artifact_version" => Field::<VersionRelation, value::StringValue>::parse,
        "checksum" => Field::<ChecksumRelation, value::StringValue>::parse,
        "checksum_algorithm" => Field::<ChecksumAlgorithmRelation, value::StringValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
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
        "primary" => Field::<PrimaryArtifactRelation, ArtifactRefOrDefinition<V>>::parse,
        "dependencies" => List::<DependencyArtifactRelation, ArtifactRefOrDefinition<V>>::parse,
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
                ast.add_edge(
                    implementation,
                    n,
                    crate::Relation::from(crate::PrimaryArtifactRelation).into(),
                );
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

impl<E, R, V> toto_parser::EntityParser<E, R> for ArtifactRefOrDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        if toto_yaml::as_string(n, ast).is_some() {
            return Some(n);
        }

        let artifact = add_with_loc(crate::Entity::from(crate::ArtifactEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| ArtifactDefinition::<V>::parse_schema(artifact, items, ast))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });

        Some(artifact)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ArtifactTypeDefinition<V>
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
