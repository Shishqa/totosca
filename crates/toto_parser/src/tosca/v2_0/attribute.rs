use std::marker::PhantomData;

use toto_ast::Parse;

use crate::{
    parse::{ParseError, ParseLoc, StaticSchema, StaticSchemaMap},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        ToscaDefinitionsVersion,
    },
};

use super::Value;

pub type AttributeAssignment<E, R, V> = Value<E, R, V>;

#[derive(Debug)]
pub struct AttributeDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for AttributeDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for AttributeDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Property;
    const SCHEMA: StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::NodeType.into());
            ast.add_edge(r, t, toto_tosca::Relation::HasType.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "description" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Description.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "metadata" => |r, n, ast|{
            let t = ast.add_node(toto_tosca::Entity::Metadata.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "status" => |r, n, ast|{
            let t = ast.add_node(toto_tosca::Entity::Value.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "default" => |r, n, ast|{
            let t = ast.add_node(toto_tosca::Entity::Value.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "validation" => |r, n, ast|{
            let t = ast.add_node(toto_tosca::Entity::Value.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "key_schema" => |r, n, ast|{
            let v = V::SchemaDefinition::from(n).parse(ast);
            ast.add_edge(r, v, toto_tosca::Relation::KeySchema.into());
        },
        "entry_schema" => |r, n, ast|{
            let v = V::SchemaDefinition::from(n).parse(ast);
            ast.add_edge(r, v, toto_tosca::Relation::EntrySchema.into());
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for AttributeDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = &ast[self.0];
        let t = t.as_yaml().unwrap();

        match t {
            toto_yaml::Entity::Map(_) => Self::parse_schema(self.0, ast),
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map").into());
                ast.add_edge(e, self.0, ParseLoc.into());

                self.0
            }
        }
    }
}
