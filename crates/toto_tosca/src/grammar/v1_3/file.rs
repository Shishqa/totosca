use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{
    grammar::{collection::Collection, field::Field, list::List, v2_0, ToscaDefinitionsVersion},
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};

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
        "namespace" => Field::<crate::ProfileRelation, v2_0::value::StringValue>::parse,
        "metadata" => Collection::<crate::MetadataRelation, v2_0::value::AnyValue>::parse,
        "description" => Field::<crate::DescriptionRelation, v2_0::value::StringValue>::parse,
        "dsl_definitions" => |_, _, _| {},
        // todo: repositories
        "imports" => List::<crate::ImportRelation, V::ImportDefinition>::parse,
        "artifact_types" => Collection::<crate::TypeRelation, V::ArtifactTypeDefinition>::parse,
        "data_types" => Collection::<crate::TypeRelation, V::DataTypeDefinition>::parse,
        "capability_types" => Collection::<crate::TypeRelation, V::CapabilityTypeDefinition>::parse,
        "interface_types" => Collection::<crate::TypeRelation, V::InterfaceTypeDefinition>::parse,
        "relationship_types" => Collection::<crate::TypeRelation, V::RelationshipTypeDefinition>::parse,
        "node_types" => Collection::<crate::TypeRelation, V::NodeTypeDefinition>::parse,
        "group_types" => Collection::<crate::TypeRelation, V::GroupTypeDefinition>::parse,
        "policy_types" => Collection::<crate::TypeRelation, V::PolicyTypeDefinition>::parse,
        "topology_template" => Field::<crate::ServiceTemplateRelation, V::ServiceTemplateDefinition>::parse,
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
