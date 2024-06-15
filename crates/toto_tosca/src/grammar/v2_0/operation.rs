use std::{marker::PhantomData};

use toto_parser::{RelationParser, Schema};

use crate::{
    grammar::{collection::Collection, field::Field, ToscaDefinitionsVersion},
    AssignmentRelation,
    DefinitionRelation, DescriptionRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
};

use super::{value};

#[derive(Debug)]
pub struct OperationDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct OperationAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for OperationDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::OperationEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "implementation" => Field::<DefinitionRelation, V::ImplementationDefinition>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "outputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for OperationAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::OperationEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "implementation" => Field::<DefinitionRelation, V::ImplementationDefinition>::parse,
        "inputs" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        "outputs" => Collection::<AssignmentRelation, value::AnyValue>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for OperationDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for OperationAssignment<V>
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
