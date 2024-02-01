use toto_tosca::Entity;
use url::Url;

use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

pub struct Reference;

impl Parse for Reference {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let s = String::from_yaml(n)
            .map_err(|err| ctx.errors.push(Box::new(err)))
            .unwrap_or_default();
        ctx.graph.add_node(Entity::Ref(s))
    }
}

impl Parse for Url {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let url = String::from_yaml(n)
            .and_then(|s| {
                Url::parse(s.as_ref()).map_err(|err| ParseError {
                    pos: Some(n.pos()),
                    error: ParseErrorKind::Custom(err.to_string()),
                })
            })
            .map_err(|err| ctx.errors.push(Box::new(err)))
            .unwrap_or_else(|()| Url::parse("file:///").unwrap());

        ctx.graph.add_node(Entity::Url(url))
    }
}
