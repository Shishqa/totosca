use std::marker::PhantomData;

use toto_tosca::{Boolean, Entity, Relation};

use super::{parse_collection, value::Value, Reference};
use crate::{
    parse::{ParseError, ParseErrorKind, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        Parse, ToscaDefinitionsVersion,
    },
};

pub type PropertyAssignment = Value;

#[derive(Debug)]
pub struct PropertyDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for PropertyDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for PropertyDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Property;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "type" => |r, n, ast| {
            has_type = true;
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Type);
        },
        "description" => |r, n, ast|{
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "metadata" => |r, n, ast|{
            parse_collection::<String, V>(ctx, root, entry.1);
        },
        "status" => |r, n, ast|{
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Status);
        },
        "default" => |r, n, ast|{
            let t = V::Value::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Default);
        },
        "validation" => |r, n, ast|{
            let t = V::Value::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Default);
        },
        "key_schema" => |r, n, ast|{
            let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::KeySchema);
        },
        "entry_schema" => |r, n, ast|{
            let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::EntrySchema);
        },
        "required" => |r, n, ast|{
            let t = Boolean::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Required);
        },
        "value" => |r, n, ast|{
            let t = V::Value::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Value);
        },
        "external-schema" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::ExternalSchema);
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for PropertyDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = &ast[self.0];
        let t = t.as_yaml().unwrap();

        match t {
            toto_yaml::Entity::Map => Self::parse_schema(self.0, ast),
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map").into());
                ast.add_edge(e, self.0, ParseLoc.into());

                self.0
            }
        }
    }
}
