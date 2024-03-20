use std::marker::PhantomData;

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};
use toto_parser::RelationParser;

use super::value;

#[derive(Debug)]
pub struct NodeTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct NodeTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

pub struct NodeTemplate;
impl<R> toto_parser::Linker<String, R> for NodeTemplate
where
    R: ToscaCompatibleRelation,
{
    const L: fn(String) -> R = |v| crate::Relation::NodeTemplate(v).into();
}

pub struct NodeType;
impl<R> toto_parser::Linker<String, R> for NodeType
where
    R: ToscaCompatibleRelation,
{
    const L: fn(String) -> R = |v| crate::Relation::NodeType(v).into();
}

pub struct RefType;
impl<R> toto_parser::Linker<(), R> for RefType
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::RefType.into();
}

pub struct DerivedFrom;
impl<R> toto_parser::Linker<(), R> for DerivedFrom
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::DerivedFrom.into();
}

impl<E, R, V> toto_parser::Schema<E, R> for NodeTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Definition.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => toto_parser::Field::<RefType, value::String>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for NodeTemplateDefinition<V>
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

impl<E, R, V> toto_parser::Schema<E, R> for NodeTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Definition.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => toto_parser::Field::<RefType, value::String>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for NodeTypeDefinition<V>
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
