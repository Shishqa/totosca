use std::marker::PhantomData;

use crate::{
    grammar::{
        collection::Collection, field::Field, field_ref::TypeRef, list::ListRelator,
        ToscaDefinitionsVersion,
    },
    DefinitionRelation, DescriptionRelation, MetadataRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, VersionRelation,
};
use toto_parser::RelationParser;

use super::v2_0::value;

#[derive(Debug)]
pub struct RelationshipTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

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
        "valid_target_types" => ListRelator::<TypeRef<crate::CapabilityEntity, crate::ValidCapabilityTypeRelation>>::parse,
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
