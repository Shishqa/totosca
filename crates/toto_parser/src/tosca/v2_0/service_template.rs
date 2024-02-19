use std::marker::PhantomData;

use toto_ast::Parse;

use super::Collection;
use crate::{
    parse::{add_error, ParseError, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        ToscaDefinitionsVersion,
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
            let t = ast.add_node(toto_tosca::Entity::Description.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "inputs" => |r, n, ast| {
            Collection::<E, R, V::ParameterDefinition>(
                n,
                r,
                PhantomData::default(),
            ).parse(ast);
        },
        "outputs" => |r, n, ast| {
            Collection::<E, R, V::ParameterDefinition>(
                n,
                r,
                PhantomData::default(),
            ).parse(ast);
        },
        "node_templates" => |r, n, ast| {
            Collection::<E, R, V::NodeTemplateDefinition>(
                n,
                r,
                PhantomData::default(),
            ).parse(ast);
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
