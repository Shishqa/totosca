use std::marker::PhantomData;

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        field_ref::TypeRef,
        list::{KeyedList, List},
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, DefinitionRelation, DescriptionRelation, DirectiveRelation,
    MetadataRelation, OrderedAssignmentRelation, OrderedDefinitionRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, VersionRelation,
};
use toto_parser::RelationParser;

use super::value;

#[derive(Debug)]
pub struct NodeTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct NodeTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for NodeTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::NodeEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => TypeRef::<crate::NodeEntity, crate::DerivedFromRelation>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "capabilities" => Collection::<DefinitionRelation, V::CapabilityDefinition>::parse,
        "requirements" => KeyedList::<OrderedDefinitionRelation, V::RequirementDefinition>::parse,
        "interfaces" => Collection::<DefinitionRelation, V::InterfaceDefinition>::parse,
        "artifacts" => Collection::<DefinitionRelation, V::ArtifactDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for NodeTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::NodeEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::NodeEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "directives" => List::<DirectiveRelation, value::StringValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "capabilities" => Collection::<AssignmentRelation, V::CapabilityAssignment>::parse,
        "requirements" => KeyedList::<OrderedAssignmentRelation, V::RequirementAssignment>::parse,
        "interfaces" => Collection::<AssignmentRelation, V::InterfaceAssignment>::parse,
        "artifacts" => Collection::<DefinitionRelation, V::ArtifactDefinition>::parse,
        "count" => |_, _, _| {},
        "node_filter" => |_, _, _| {},
        "copy" => |_, _, _| {},
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for NodeTemplateDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for NodeTypeDefinition<V>
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
