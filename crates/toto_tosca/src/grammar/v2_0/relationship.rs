use std::{collections::HashSet, marker::PhantomData};

use crate::{
    grammar::{
        collection::Collection, field::Field, field_ref::FieldRef, list::List,
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, DefinitionRelation, DescriptionRelation, MetadataRelation,
    RefValidCapabilityTypeRelation, RefValidSourceNodeTypeRelation, RefValidTargetNodeTypeRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, VersionRelation,
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
        "derived_from" => |r, n, ast| FieldRef::derived_from(crate::Entity::from(crate::RelationshipEntity)).parse(r, n, ast),
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
        "type" => |r, n, ast| FieldRef::has_type(crate::Entity::from(crate::RelationshipEntity)).parse(r, n, ast),
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
        "type" => |r, n, ast| FieldRef::has_type(crate::Entity::from(crate::RelationshipEntity)).parse(r, n, ast),
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
        "type" => |r, n, ast| FieldRef::has_type(crate::Entity::from(crate::RelationshipEntity)).parse(r, n, ast),
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
                FieldRef::has_type(crate::Entity::from(crate::RelationshipEntity))
                    .parse(rel, n, ast);
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
                FieldRef::has_type(crate::Entity::from(crate::RelationshipEntity))
                    .parse(rel, n, ast);
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
