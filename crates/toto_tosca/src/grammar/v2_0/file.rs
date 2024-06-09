use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    DescriptionRelation, ImportRelation, MetadataRelation, ProfileRelation,
    ServiceTemplateRelation, ToscaCompatibleEntity, ToscaCompatibleRelation, TypeRelation,
};

use super::value;

#[derive(Debug)]
pub struct ToscaFileDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::FileEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "tosca_definitions_version" => |_, _, _| {},
        "dsl_definitions" => |_, _, _| {},
        "profile" => Field::<ProfileRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "imports" => List::<ImportRelation, V::ImportDefinition>::parse,
        "service_template" => Field::<ServiceTemplateRelation, V::ServiceTemplateDefinition>::parse,
        "node_types" => Collection::<TypeRelation, V::NodeTypeDefinition>::parse,
        "data_types" => Collection::<TypeRelation, V::DataTypeDefinition>::parse,
        "artifact_types" => Collection::<TypeRelation, V::ArtifactTypeDefinition>::parse,
        "capability_types" => Collection::<TypeRelation, V::CapabilityTypeDefinition>::parse,
        "relationship_types" => Collection::<TypeRelation, V::RelationshipTypeDefinition>::parse,
        "group_types" => Collection::<TypeRelation, V::GroupTypeDefinition>::parse,
        "policy_types" => Collection::<TypeRelation, V::PolicyTypeDefinition>::parse,
        "interface_types" => Collection::<TypeRelation, V::InterfaceTypeDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ToscaFileDefinition<V>
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
