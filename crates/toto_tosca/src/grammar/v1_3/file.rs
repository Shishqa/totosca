use std::marker::PhantomData;

use toto_parser::{add_with_loc, RelationParser, Schema};

use crate::{
    grammar::{v2_0, ToscaDefinitionsVersion},
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};

#[derive(Debug)]
pub struct ToscaFileDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> Schema<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
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
        let file = add_with_loc(crate::Entity::File, n, ast);
        toto_yaml::as_map(n, ast)
            .or_else(|| {
                add_with_loc(toto_parser::ParseError::UnexpectedType("map"), n, ast);
                None
            })
            .and_then(|items| {
                toto_parser::parse_schema(&Self::SCHEMA, file, items, ast);
                Some(file)
            });
        Some(file)
    }
}
