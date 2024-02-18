use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use super::parse_collection;
use crate::{
    parse::{add_error, ParseError, ParseErrorKind, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        Parse, ToscaDefinitionsVersion,
    },
};

#[derive(Debug)]
pub struct ServiceTemplateDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(V, E, R)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for ServiceTemplateDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for ServiceTemplateDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::ServiceTemplate;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "description" => |r, n, ast| {
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "inputs" => |r, n, ast| {
            parse_collection::<V::ParameterDefinition, V>(ctx, root, entry.1);
        },
        "outputs" => |r, n, ast| {
            parse_collection::<V::ParameterDefinition, V>(ctx, root, entry.1);
        },
        "node_templates" => |r, n, ast| {
            parse_collection::<V::NodeTemplateDefinition, V>(ctx, root, entry.1);
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for ServiceTemplateDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        if let toto_yaml::Entity::Map = t {
            Self::parse_schema(self.0, ast)
        } else {
            add_error(self.0, ast, ParseError::UnexpectedType("map"));
            self.0
        }
    }
}
