use std::marker::PhantomData;

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};
use toto_parser::RelationParser;

use super::value;

#[derive(Debug)]
pub struct NodeTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct NodeTemplateDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

pub struct Definition;
impl<R> toto_parser::Linker<String, R> for Definition
where
    R: ToscaCompatibleRelation,
{
    const L: fn(String) -> R = |v| crate::Relation::Definition(v).into();
}

pub struct TypeDefinition;
impl<R> toto_parser::Linker<String, R> for TypeDefinition
where
    R: ToscaCompatibleRelation,
{
    const L: fn(String) -> R = |v| crate::Relation::Type(v).into();
}

pub struct RefHasType;
impl<R> toto_parser::Linker<(), R> for RefHasType
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::RefHasType.into();
}

pub struct RefDerivedFrom;
impl<R> toto_parser::Linker<(), R> for RefDerivedFrom
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::RefDerivedFrom.into();
}

impl<E, R, V> toto_parser::Schema<E, R> for NodeTemplateDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Node.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => toto_parser::Field::<RefHasType, value::String>::parse,
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
    const SELF: fn() -> E = || crate::Entity::Node.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => toto_parser::Field::<RefDerivedFrom, value::String>::parse,
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
