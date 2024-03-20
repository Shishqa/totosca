use std::marker::PhantomData;

use toto_parser::RelationParser;

use crate::{
    grammar::{v2_0, ToscaDefinitionsVersion},
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
    const SELF: fn() -> E = || crate::Entity::File.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "tosca_definitions_version" => |_, _, _| {},
        "description" => toto_parser::Field::<v2_0::value::Description, v2_0::value::String>::parse,
        "imports" => toto_parser::List::<v2_0::import::Import, V::ImportDefinition>::parse,
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
