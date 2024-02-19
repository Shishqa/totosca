use std::marker::PhantomData;

use crate::{
    parse::{ParseError, ParseLoc, StaticSchema},
    tosca::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ImportDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(E, R, V)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for ImportDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for ImportDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Import;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "url" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Url.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "profile" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Profile.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "repository" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Repository.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "namespace" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Namespace.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for ImportDefinition<E, R, V>
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
                let root = ast.add_node(toto_tosca::Entity::Import.into());
                ast.add_edge(root, self.0, ParseLoc.into());

                let t = ast.add_node(toto_tosca::Entity::Url.into());
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
