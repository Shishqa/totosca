use std::{collections::HashSet, marker::PhantomData};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    AssignmentRelation, DefinitionRelation, DescriptionRelation, MetadataRelation,
    RefDerivedFromRelation, RefHasTypeRelation, RefValidCapabilityTypeRelation,
    RefValidSourceNodeTypeRelation, RefValidTargetNodeTypeRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, VersionRelation,
};
use toto_parser::{mandatory, RelationParser};

use super::value;

#[derive(Debug)]
pub struct RelationshipTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct RelationshipTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for RelationshipTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RelationshipEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => Field::<RefDerivedFromRelation, value::StringValue>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "interfaces" => Collection::<DefinitionRelation, V::InterfaceDefinition>::parse,
        "valid_capability_types" => List::<RefValidCapabilityTypeRelation, value::StringValue>::parse,
        "valid_target_node_types" => List::<RefValidTargetNodeTypeRelation, value::StringValue>::parse,
        "valid_source_node_types" => List::<RefValidSourceNodeTypeRelation, value::StringValue>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for RelationshipTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RelationshipEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => Field::<RefHasTypeRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "interfaces" => Collection::<AssignmentRelation, V::InterfaceAssignment>::parse,
        "copy" => |_, _, _| {},
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for RelationshipTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for RelationshipTemplateDefinition<V>
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
