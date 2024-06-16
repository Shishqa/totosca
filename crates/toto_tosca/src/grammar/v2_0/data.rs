use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser};

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        field_ref::{FieldRef, TypeRef},
        ToscaDefinitionsVersion,
    },
    DefaultRelation, DefinitionRelation, DescriptionRelation, EntrySchemaRelation,
    ExternalSchemaRelation, KeySchemaRelation, MappingRelation, MetadataRelation, RequiredRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, ValidationRelation, ValueRelation,
    VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct DataTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct SchemaDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct PropertyDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct AttributeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct ParameterDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for DataTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => TypeRef::<crate::DataEntity, crate::DerivedFromRelation>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "validation" => Field::<ValidationRelation, value::AnyValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "key_schema" => Field::<KeySchemaRelation, V::SchemaDefinition>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, V::SchemaDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for SchemaDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::DataEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "validation" => Field::<ValidationRelation, value::AnyValue>::parse,
        "key_schema" => Field::<KeySchemaRelation, V::SchemaDefinition>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, V::SchemaDefinition>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

pub struct StatusValue;
impl<E, R> toto_parser::EntityParser<E, R> for StatusValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        let s = toto_yaml::as_string(n, ast);
        if s.is_none() {
            add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
            return None;
        }
        let s = s.unwrap();

        let status = match s.0.as_str() {
            "supported" => Ok(crate::StatusEntity::Supported),
            "unsupported" => Ok(crate::StatusEntity::Unsupported),
            "experimental" => Ok(crate::StatusEntity::Experimental),
            "deprecated" => Ok(crate::StatusEntity::Deprecated),
            other => Err(ParseError::Custom(format!("unknown status: {}", other))),
        };

        match status {
            Ok(status) => Some(add_with_loc(crate::Entity::from(status), n, ast)),
            Err(err) => {
                add_with_loc(err, n, ast);
                None
            }
        }
    }
}

impl<E, R, V> toto_parser::Schema<E, R> for AttributeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::DataEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "default" => Field::<DefaultRelation, value::AnyValue>::parse,
        "status" => Field::<DefaultRelation, StatusValue>::parse,
        "validation" => Field::<ValidationRelation, value::AnyValue>::parse,
        "key_schema" => Field::<KeySchemaRelation, V::SchemaDefinition>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, V::SchemaDefinition>::parse,
        "metadata" => Collection::<MetadataRelation, value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::Schema<E, R> for PropertyDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::DataEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "required" => Field::<RequiredRelation, value::BoolValue>::parse,
        "default" => Field::<DefaultRelation, value::AnyValue>::parse,
        "status" => Field::<DefaultRelation, StatusValue>::parse,
        "validation" => Field::<ValidationRelation, value::AnyValue>::parse,
        "value" => Field::<ValueRelation, value::AnyValue>::parse,
        "key_schema" => Field::<KeySchemaRelation, V::SchemaDefinition>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, V::SchemaDefinition>::parse,
        "external-schema" => Field::<ExternalSchemaRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::Schema<E, R> for ParameterDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => TypeRef::<crate::DataEntity, crate::HasTypeRelation>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "required" => Field::<RequiredRelation, value::BoolValue>::parse,
        "default" => Field::<DefaultRelation, value::AnyValue>::parse,
        "status" => Field::<DefaultRelation, StatusValue>::parse,
        "validation" => Field::<ValidationRelation, value::AnyValue>::parse,
        "value" => Field::<ValueRelation, value::AnyValue>::parse,
        "mapping" => Field::<MappingRelation, value::AnyValue>::parse,
        "key_schema" => Field::<KeySchemaRelation, V::SchemaDefinition>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, V::SchemaDefinition>::parse,
        "external-schema" => Field::<ExternalSchemaRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::StringValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for DataTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for SchemaDefinition<V>
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
                let data = add_with_loc(crate::Entity::from(crate::DataEntity), n, ast);
                TypeRef::<crate::DataEntity, crate::HasTypeRelation>::parse(data, n, ast);
                Some(data)
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

impl<E, R, V> toto_parser::EntityParser<E, R> for AttributeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        // TODO: short notation
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for PropertyDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        // TODO: short notation
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ParameterDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        // TODO: short notation
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}
