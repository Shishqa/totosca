use std::marker::PhantomData;

use toto_ast::RelationParser;
use toto_parser::{add_with_loc, Schema};

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{import, value};

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
        "profile" => toto_parser::Field::<value::Profile, value::String>::parse,
        "metadata" => toto_parser::Collection::<value::Metadata, value::String>::parse,
        "description" => toto_parser::Field::<value::Description, value::String>::parse,
        "imports" => toto_parser::List::<import::Import, V::ImportDefinition>::parse,
    };
}

impl<E, R, V> toto_ast::EntityParser<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        let file = add_with_loc(crate::Entity::File, n, ast);
        if let Some(items) = toto_yaml::as_map(n, ast).or_else(|| {
            add_with_loc(toto_parser::ParseError::UnexpectedType("map"), n, ast);
            None
        }) {
            Self::parse_schema(file, items, ast);
        }
        Some(file)
    }
}
