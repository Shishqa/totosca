use std::marker::PhantomData;

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};
use toto_parser::RelationParser;

use super::node;

#[derive(Debug)]
pub struct ServiceTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

pub struct ServiceTemplate;
impl<R> toto_parser::Linker<(), R> for ServiceTemplate
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::ServiceTemplate.into();
}

impl<E, R, V> toto_parser::Schema<E, R> for ServiceTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Definition.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "node_templates" => toto_parser::Collection::<node::NodeTemplate, V::NodeTemplateDefinition>::parse,
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
