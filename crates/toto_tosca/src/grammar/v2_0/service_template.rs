use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{
    grammar::{collection::Collection, ToscaDefinitionsVersion},
    DefinitionRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
};

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
        "node_templates" => Collection::<DefinitionRelation, V::NodeTemplateDefinition>::parse,
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
