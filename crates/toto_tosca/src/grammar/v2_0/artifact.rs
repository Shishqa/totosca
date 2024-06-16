use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, EntityParser, RelationParser};

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        field_ref::{DefRef, FieldRef, TypeRef},
        list::{List, ListRelator},
        ToscaDefinitionsVersion,
    },
    ArtifactEntity, AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation,
    DefinitionRelation, DependencyArtifactRelation, DerivedFromRelation, DescriptionRelation,
    FileExtRelation, HasFileRelation, HasTypeRelation, MetadataRelation, MimeTypeRelation,
    PrimaryArtifactRelation, RepositoryRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct ArtifactTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ArtifactDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ImplementationDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ArtifactRefOrDefinition<V: ToscaDefinitionsVersion, Rel>(PhantomData<(V, Rel)>);

impl<E, R, V> toto_parser::Schema<E, R> for ArtifactTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ArtifactEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => TypeRef::<ArtifactEntity, DerivedFromRelation>::parse,
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
        "type" => TypeRef::<ArtifactEntity, HasTypeRelation>::parse,
        "file" => Field::<HasFileRelation, value::StringValue>::parse,
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
        "primary" => ArtifactRefOrDefinition::<V, PrimaryArtifactRelation>::parse,
        "dependencies" => ListRelator::<ArtifactRefOrDefinition<V, DependencyArtifactRelation>>::parse,
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
        match ast.node_weight(n).unwrap().as_yaml() {
            Some(toto_yaml::Entity::Map(_)) => <Self as toto_parser::Schema<E, R>>::parse(n, ast),
            Some(toto_yaml::Entity::Str(_) | toto_yaml::Entity::Null(_)) => {
                let implementation =
                    add_with_loc(crate::Entity::from(crate::ImplementationEntity), n, ast);
                DefRef::<
                    crate::NodeEntity,
                    crate::ArtifactEntity,
                    crate::PrimaryArtifactRelation,
                >::parse(implementation, n, ast);
                Some(implementation)
            }
            _ => {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            }
        }
    }
}

impl<E, R, V, Rel> toto_parser::ValueRelationParser<E, R, usize> for ArtifactRefOrDefinition<V, Rel>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
    Rel: Default,
    crate::Relation: From<Rel>,
{
    fn parse(
        _: usize,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) {
        <Self as toto_parser::RelationParser<E, R>>::parse(root, n, ast);
    }
}

impl<E, R, V, Rel> toto_parser::RelationParser<E, R> for ArtifactRefOrDefinition<V, Rel>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
    Rel: Default,
    crate::Relation: From<Rel>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        match ast.node_weight(n).expect("node not found").as_yaml() {
            Some(toto_yaml::Entity::Map(_)) => {
                V::ArtifactDefinition::parse(n, ast);
            }
            Some(toto_yaml::Entity::Str(_) | toto_yaml::Entity::Null(_)) => {
                DefRef::<crate::NodeEntity, crate::ArtifactEntity, Rel>::parse(root, n, ast);
            }
            _ => {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
            }
        };
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
