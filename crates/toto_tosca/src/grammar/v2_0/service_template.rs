use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{
    grammar::{collection::Collection, field::Field, ToscaDefinitionsVersion},
    DefinitionRelation, DescriptionRelation, MetadataRelation, SubstitutionMappingRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};

use super::value;

#[derive(Debug)]
pub struct ServiceTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ServiceTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ServiceTemplateEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "outputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "node_templates" => Collection::<DefinitionRelation, V::NodeTemplateDefinition>::parse,
        "relationship_templates" => Collection::<DefinitionRelation, V::RelationshipTemplateDefinition>::parse,
        "groups" => Collection::<DefinitionRelation, V::GroupDefinition>::parse,
        "workflows" => Collection::<DefinitionRelation, V::WorkflowDefinition>::parse,
        "policies" => |_, _, _| {},
        "substitution_mappings" => Field::<SubstitutionMappingRelation, V::SubstitutionMapping>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ServiceTemplateDefinition<V>
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
