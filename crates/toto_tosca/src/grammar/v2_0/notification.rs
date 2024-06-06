use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefaultRelation,
    DefinitionRelation, DependencyArtifactRelation, DescriptionRelation, EntrySchemaRelation,
    ExternalSchemaRelation, FileExtRelation, KeySchemaRelation, MappingRelation, MetadataRelation,
    MimeTypeRelation, PrimaryArtifactRelation, RefDerivedFromRelation, RefHasFileRelation,
    RefHasTypeRelation, RepositoryRelation, RequiredRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, ValidationRelation, ValueRelation, VersionRelation,
};

use super::{value, ImplementationDefinition};

#[derive(Debug)]
pub struct NotificationDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct NotificationAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for NotificationDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::NotificationEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "implementation" => Field::<DefinitionRelation, ImplementationDefinition<V>>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "outputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for NotificationAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::NotificationEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "implementation" => Field::<DefinitionRelation, ImplementationDefinition<V>>::parse,
        "inputs" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "outputs" => Collection::<AssignmentRelation, value::AnyValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for NotificationDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for NotificationAssignment<V>
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
