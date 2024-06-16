use std::{collections::HashSet, marker::PhantomData};

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        field_ref::{FieldRef, TypeRef},
        list::{List, ListRelator},
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, DefinitionRelation, DescriptionRelation, MetadataRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, ValidCapabilityTypeRelation,
    ValidSourceNodeTypeRelation, ValidTargetNodeTypeRelation, VersionRelation,
};
use toto_parser::{add_with_loc, mandatory, RelationParser};

use super::value;

#[derive(Debug)]
pub struct RelationshipTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct RelationshipTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct RelationshipDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct RelationshipAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for RelationshipTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RelationshipEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => TypeRef::<crate::RelationshipEntity, crate::DerivedFromRelation>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "attributes" => Collection::<DefinitionRelation, V::AttributeDefinition>::parse,
        "interfaces" => Collection::<DefinitionRelation, V::InterfaceDefinition>::parse,
        "valid_capability_types" => ListRelator::<TypeRef<crate::CapabilityEntity, crate::ValidCapabilityTypeRelation>>::parse,
        "valid_target_node_types" => ListRelator::<TypeRef<crate::NodeEntity, crate::ValidTargetNodeTypeRelation>>::parse,
        "valid_source_node_types" => ListRelator::<TypeRef<crate::NodeEntity, crate::ValidSourceNodeTypeRelation>>::parse,
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
        "type" => TypeRef::<crate::RelationshipEntity, crate::HasTypeRelation>::parse,
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

impl<E, R, V> toto_parser::Schema<E, R> for RelationshipDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RelationshipEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::RelationshipEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "interfaces" => Collection::<AssignmentRelation, V::InterfaceAssignment>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::Schema<E, R> for RelationshipAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RelationshipEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::RelationshipEntity, crate::HasTypeRelation>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "attributes" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "interfaces" => Collection::<AssignmentRelation, V::InterfaceAssignment>::parse,
    };
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

impl<E, R, V> toto_parser::EntityParser<E, R> for RelationshipDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let rel = add_with_loc(crate::Entity::from(crate::RelationshipEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(rel, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                FieldRef::type_ref(crate::RelationshipEntity, crate::HasTypeRelation)
                    .link(rel, n, ast);
                rel
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(rel)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for RelationshipAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let rel = add_with_loc(crate::Entity::from(crate::RelationshipEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(rel, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                FieldRef::type_ref(crate::RelationshipEntity, crate::HasTypeRelation)
                    .link(rel, n, ast);
                rel
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(rel)
    }
}
