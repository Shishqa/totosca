use toto_tosca::{Entity, Relation};

use crate::parse::{Error, FromYaml, GraphHandle, Parse, ParseError};

#[derive(Debug)]
pub struct ImportDefinition;

impl Parse for ImportDefinition {
    fn parse(ctx: &mut crate::parse::Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::ServiceTemplate);

        let mut has_url: bool = false;
        let mut has_profile: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "url" => {
                        String::from_yaml(entry.1)
                            .map_err(|err| ctx.errors.push(err))
                            .map(|r| {
                                let t = ctx.graph.add_node(Entity::Ref(r));
                                ctx.graph.add_edge(root, t, Relation::Url);
                            });
                    }
                    "profile" => {
                        String::from_yaml(entry.1)
                            .map_err(|err| ctx.errors.push(err))
                            .map(|r| {
                                let t = ctx.graph.add_node(Entity::Ref(r));
                                ctx.graph.add_edge(root, t, Relation::Profile);
                            });
                    }
                    "repository" => {
                        String::from_yaml(entry.1)
                            .map_err(|err| ctx.errors.push(err))
                            .map(|r| {
                                let t = ctx.graph.add_node(Entity::Ref(r));
                                ctx.graph.add_edge(root, t, Relation::Repository);
                            });
                    }
                    "namespace" => {
                        let t = String::parse(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Namespace);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(s) = n.as_str() {
            let t = ctx.graph.add_node(Entity::Ref(s.to_string()));
            ctx.graph.add_edge(root, t, Relation::Url);
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
            });
            root;
        }

        if !has_url && !has_profile {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("url or profile"),
            });
        } else if has_url && has_profile {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::Custom("url and profile fields are mutually exclusive"),
            });
        }

        root
    }
}
