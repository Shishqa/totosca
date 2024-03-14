use std::marker::PhantomData;

use crate::{
    parse::{
        self, add_error, parse_schema, EntityParser, ParseError, ParseLoc, RelationParser, Schema,
        StaticSchemaMap,
    },
    tosca::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion},
};

use super::value::{self, Linker};

pub struct Import;
impl Linker<usize> for Import {
    const L: fn(v: usize) -> toto_tosca::Relation = toto_tosca::Relation::Import;
}

pub struct Url;
impl Linker<()> for Url {
    const L: fn(v: ()) -> toto_tosca::Relation = |_| toto_tosca::Relation::Url;
}

#[derive(Debug)]
pub struct ImportDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> Schema<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion,
{
    const SCHEMA: StaticSchemaMap<E, R> = phf::phf_map! {
        "url" => value::Field::<Url, value::String>::parse::<E, R>,
        // "profile" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Profile.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "repository" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Repository.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "namespace" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Namespace.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
    };
}

impl<V> EntityParser<toto_ast::GraphHandle> for ImportDefinition<V>
where
    V: ToscaDefinitionsVersion,
{
    fn parse<E, R>(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let import = ast.add_node(toto_tosca::Entity::Definition.into());
        toto_yaml::as_map(n, ast)
            .and_then(|items| Some(parse_schema(&Self::SCHEMA, import, items, ast)))
            .or(toto_yaml::as_string(n, ast).and_then(|url_str| {
                ast.add_edge(import, n, toto_tosca::Relation::Url.into());
                Some(import)
            }))
            .or_else(|| {
                add_error(n, ast, ParseError::UnexpectedType("map or string"));
                None
            });
        Some(import)
    }
}
