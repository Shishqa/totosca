use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseErrorKind, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        Parse, ToscaDefinitionsVersion,
    },
};

use super::{parse_collection, parse_keyed_list_collection, Reference};

#[derive(Debug)]
pub struct NodeType<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for NodeType<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for NodeType<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::NodeType;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "derived_from" => |r, n, ast| {
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::DerivedFrom);
        },
        "description" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "metadata" => |r, n, ast| {
            parse_collection::<String, V>(ctx, root, entry.1);
        },
        "version" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Version);
        },
        "properties" => |r, n, ast| {
            parse_collection::<V::PropertyDefinition, V>(ctx, root, entry.1);
        },
        "attributes" => |r, n, ast| {
            parse_collection::<V::AttributeDefinition, V>(ctx, root, entry.1);
        },
        "requirements" => |r, n, ast| {
            parse_keyed_list_collection::<V::RequirementDefinition, V>(
                ctx, root, entry.1,
            );
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for NodeType<E, R, V>
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

#[derive(Debug)]
pub struct NodeTemplate<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for NodeTemplate<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for NodeTemplate<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Node;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "type" => |r, n, ast| {
            has_type = true;
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Type);
        },
        "description" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "metadata" => |r, n, ast|{
            parse_collection::<String, V>(ctx, root, entry.1);
        },
        "properties" => |r, n, ast|{
            parse_collection::<V::PropertyAssignment, V>(ctx, root, entry.1);
        },
        "attributes" => |r, n, ast|{
            parse_collection::<V::AttributeAssignment, V>(ctx, root, entry.1);
        },
        "requirements" => |r, n, ast|{
            parse_keyed_list_collection::<V::RequirementDefinition, V>(
                ctx, root, entry.1,
            );
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for NodeTemplate<E, R, V>
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
