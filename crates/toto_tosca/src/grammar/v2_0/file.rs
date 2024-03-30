use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{import, node, service_template, value};

#[derive(Debug)]
pub struct ToscaFileDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::File.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "tosca_definitions_version" => |_, _, _| {},
        "profile" => toto_parser::Field::<value::Profile, value::String>::parse,
        "metadata" => toto_parser::Collection::<value::Metadata, value::String>::parse,
        "description" => toto_parser::Field::<value::Description, value::String>::parse,
        "imports" => toto_parser::List::<import::Import, V::ImportDefinition>::parse,
        "service_template" => toto_parser::Field::<service_template::ServiceTemplate, V::ServiceTemplateDefinition>::parse,
        "node_types" => toto_parser::Collection::<node::TypeDefinition, V::NodeTypeDefinition>::parse,
        "data_types" => toto_parser::Collection::<node::TypeDefinition, V::DataTypeDefinition>::parse,
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
