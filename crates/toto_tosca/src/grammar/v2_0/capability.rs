use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, RelationParser};

use crate::{
    grammar::{
        collection::Collection, field::Field, field_ref::FieldRef, list::List,
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, DefinitionRelation, DescriptionRelation, DirectiveRelation,
    MetadataRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    ValidRelationshipTypeRelation, ValidSourceNodeTypeRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct CapabilityTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct CapabilityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct CapabilityAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for CapabilityTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::CapabilityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => |r, n, ast| FieldRef::type_ref(crate::CapabilityEntity, crate::DerivedFromRelation).parse(r, n, ast),
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "valid_source_node_types" => List::<ValidSourceNodeTypeRelation, value::StringValue>::parse,
        "valid_relationship_types" => List::<ValidRelationshipTypeRelation, value::StringValue>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for CapabilityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::CapabilityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => |r, n, ast| FieldRef::type_ref(crate::CapabilityEntity, crate::HasTypeRelation).parse(r, n, ast),
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "valid_source_node_types" => List::<ValidSourceNodeTypeRelation, value::StringValue>::parse,
        "valid_relationship_types" => List::<ValidRelationshipTypeRelation, value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::Schema<E, R> for CapabilityAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::CapabilityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "directives" => List::<DirectiveRelation, value::StringValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for CapabilityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let capability = add_with_loc(crate::Entity::from(crate::CapabilityEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(capability, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                FieldRef::type_ref(crate::CapabilityEntity, crate::HasTypeRelation)
                    .link(capability, n, ast);
                capability
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(capability)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for CapabilityTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for CapabilityAssignment<V>
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
