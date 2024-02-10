use std::marker::PhantomData;

use petgraph::{
    data::Build,
    visit::{GraphBase, IntoEdgeReferences, IntoEdges},
};
use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseLoc},
    tosca::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ImportDefinition<A, V>(pub A::NodeId, PhantomData<V>)
where
    A: GraphBase,
    V: ToscaDefinitionsVersion<A>;

impl<A, V> toto_ast::Parse<A> for ImportDefinition<A, V>
where
    A: GraphBase + Build + IntoEdges + IntoEdgeReferences,
    A::NodeWeight: ToscaCompatibleEntity,
    A::EdgeWeight: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<A>,
{
    fn parse(self, ast: &mut A) -> A::NodeId {
        let t = &ast[n];
        let t = t.as_yaml().unwrap();

        let mut has_url: Option<A::NodeId> = None;
        let mut has_profile: Option<A::NodeId> = None;
        if let Ok(map) = t.0.as_map() {
            let root = ast.add_node(Entity::Import.into());
            ast.add_edge(root, n, ParseLoc.into());

            let keys = ast
                .edges(n)
                .filter_map(|e| match e.weight().as_yaml().unwrap() {
                    toto_yaml::Relation::MapKey => Some(e.target()),
                    _ => None,
                })
                .filter_map(|k| match &ast[k].as_yaml().unwrap().0.yaml() {
                    yaml_peg::Yaml::Str(str_key) => Some((k.clone(), str_key.clone())),
                    _ => None,
                })
                .map(|(k, str_key)| {
                    let v = ast
                        .edges(k)
                        .find_map(|e| match e.weight().as_yaml().unwrap() {
                            toto_yaml::Relation::MapValue => Some(e.target()),
                            _ => None,
                        })
                        .unwrap();

                    (str_key, k, v)
                })
                .collect::<Vec<(String, A::NodeId, A::NodeId)>>();

            keys.iter()
                .for_each(|str_key, k, v| match str_key.as_str() {
                    "url" => {
                        let t = ast.add_node(toto_tosca::Entity::Url.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                        has_url = Some(t);
                    }
                    "profile" => {
                        let t = ast.add_node(toto_tosca::Entity::Profile.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                        has_profile = Some(t);
                    }
                    "repository" => {
                        let t = ast.add_node(toto_tosca::Entity::Repository.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    "namespace" => {
                        let t = ast.add_node(toto_tosca::Entity::Namespace.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    f => {
                        let e = ast.add_node(ParseError::UnknownField(f.to_string()).into());
                        ast.add_edge(e, *k, ParseLoc.into());
                    }
                });

            if has_url.is_none() && has_profile.is_none() {
                let e = ast.add_node(ParseError::MissingField("url or profile").into());
                ast.add_edge(e, n, ParseLoc.into());
            } else if has_url.is_some() && has_profile.is_some() {
                let e = ast.add_node(
                    ParseError::Custom("url and profile fields are mutually exclusive".to_string())
                        .into(),
                );
                ast.add_edge(e, n, ParseLoc.into());
            }

            root
        } else if n.as_str().is_ok() {
            has_url = true;

            let root = ast.add_node(Entity::Import.into());
            ast.add_edge(root, n, ParseLoc.into());

            let t = ast.add_node(toto_tosca::Entity::Url.into());
            ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());

            root
        } else {
            let e = ast.add_node(ParseError::UnexpectedType("map or string").into());
            ast.add_edge(e, n, ParseLoc.into());

            n
        }
    }
}
