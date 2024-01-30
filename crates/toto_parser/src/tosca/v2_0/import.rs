use toto_tosca::{Entity, Relation};
use url::Url;

use crate::{
    parse::{Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
};

use super::Reference;

#[derive(Debug)]
pub struct ImportDefinition;

impl Parse for ImportDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut crate::parse::Context,
        n: &yaml_peg::NodeRc,
    ) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Import);

        let mut has_url: bool = false;
        let mut has_profile: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "url" => {
                        has_url = true;
                        let t = <Url as Parse>::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Url);
                    }
                    "profile" => {
                        has_profile = true;
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Profile);
                    }
                    "repository" => {
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Repository);
                    }
                    "namespace" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Namespace);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(_) = n.as_str() {
            has_url = true;
            let t = <Url as Parse>::parse::<V>(ctx, n);
            ctx.graph.add_edge(root, t, Relation::Url);
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
            });
            return root;
        }

        if !has_url && !has_profile {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("url or profile"),
            });
        } else if has_url && has_profile {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::Custom(
                    "url and profile fields are mutually exclusive".to_string(),
                ),
            });
        }

        root
    }
}
