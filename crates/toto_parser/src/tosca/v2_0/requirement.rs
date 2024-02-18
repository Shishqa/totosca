use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        ToscaDefinitionsVersion,
    },
};

use super::{List, Reference};

#[derive(Debug)]
pub struct RequirementDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for RequirementDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for RequirementDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Requirement;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "node" => |r, n, ast| {
            has_node = true;
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Node);
        },
        "description" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "count_range" => |r, n, ast| {
            let t = List::<V::Value>::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::CountRange);
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for RequirementDefinition<E, R, V>
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
            toto_yaml::Entity::Str(_) => {
                let root = ast.add_node(toto_tosca::Entity::Node.into());
                ast.add_edge(root, self.0, ParseLoc.into());

                ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                ast.add_edge(t, self.0, ParseLoc.into());

                root
            }
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map or string").into());
                ast.add_edge(e, self.0, ParseLoc.into());

                self.0
            }
        }
    }
}

#[derive(Debug)]
pub struct RequirementAssignment<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for RequirementAssignment<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for RequirementAssignment<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Requirement;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "node" => |r, n, ast| {
            has_node = true;
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Node);
        },
        "count" => |r, n, ast| {
            let t = Integer::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Count);
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for RequirementAssignment<E, R, V>
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
            toto_yaml::Entity::Str(_) => {
                let root = ast.add_node(toto_tosca::Entity::Node.into());
                ast.add_edge(root, self.0, ParseLoc.into());

                ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                ast.add_edge(t, self.0, ParseLoc.into());

                root
            }
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map or string").into());
                ast.add_edge(e, self.0, ParseLoc.into());

                self.0
            }
        }
    }
}
