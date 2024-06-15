use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{mandatory, RelationParser};

use crate::{
    grammar::{collection::Collection, field::Field, field_ref::FieldRef, ToscaDefinitionsVersion},
    AssignmentRelation, DefinitionRelation, DescriptionRelation, MetadataRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct InterfaceTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct InterfaceDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct InterfaceAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for InterfaceTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::InterfaceEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => |r, n, ast| FieldRef::type_ref(crate::InterfaceEntity, crate::DerivedFromRelation).parse(r, n, ast),
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "operations" => Collection::<DefinitionRelation, V::OperationDefinition>::parse,
        "notifications" => Collection::<DefinitionRelation, V::NotificationDefinition>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for InterfaceDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::InterfaceEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => |r, n, ast| FieldRef::type_ref(crate::InterfaceEntity, crate::HasTypeRelation).parse(r, n, ast),
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "operations" => Collection::<DefinitionRelation, V::OperationDefinition>::parse,
        "notifications" => Collection::<DefinitionRelation, V::NotificationDefinition>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::Schema<E, R> for InterfaceAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::InterfaceEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "operations" => Collection::<AssignmentRelation, V::OperationAssignment>::parse,
        "notifications" => Collection::<AssignmentRelation, V::NotificationAssignment>::parse,
        "inputs" => Collection::<AssignmentRelation, value::AnyValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for InterfaceTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for InterfaceDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for InterfaceAssignment<V>
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
