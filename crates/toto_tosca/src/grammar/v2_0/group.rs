use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefaultRelation,
    DefinitionRelation, DependencyArtifactRelation, DescriptionRelation, DirectiveRelation,
    EntrySchemaRelation, ExternalSchemaRelation, FileExtRelation, KeySchemaRelation,
    MappingRelation, MetadataRelation, MimeTypeRelation, PrimaryArtifactRelation,
    RefDerivedFromRelation, RefHasFileRelation, RefHasTypeRelation, RefMemberNodeTemplateRelation,
    RefMemberNodeTypeRelation, RefValidRelationshipTypeRelation, RefValidSourceNodeTypeRelation,
    RepositoryRelation, RequiredRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    ValidationRelation, ValueRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct GroupTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct GroupDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for GroupTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::GroupEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => Field::<RefDerivedFromRelation, value::StringValue>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "members" => List::<RefMemberNodeTypeRelation, value::StringValue>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for GroupDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::GroupEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => Field::<RefHasTypeRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "members" => List::<RefMemberNodeTemplateRelation, value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for GroupTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for GroupDefinition<V>
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
