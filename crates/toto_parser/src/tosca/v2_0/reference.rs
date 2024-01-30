use toto_tosca::Entity;
use url::Url;

use crate::{
    parse::{Context, Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

pub struct Reference;

impl Parse for Reference {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let s = String::from_yaml(n)
            .map_err(|err| ctx.errors.push(err))
            .unwrap_or_default();
        ctx.graph.add_node(Entity::Ref(s))
    }
}

impl Parse for Url {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let url = String::from_yaml(n)
            .and_then(|s| {
                Url::parse(&s).map_err(|err| Error {
                    pos: Some(n.pos()),
                    error: ParseError::Custom(err.to_string()),
                })
            })
            .map_err(|err| ctx.errors.push(err))
            .unwrap_or_else(|()| Url::parse("file:///").unwrap());

        ctx.graph.add_node(Entity::Url(url))
    }
}
