use toto_tosca::{Entity, Relation};

use super::parse_collection;
use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ServiceTemplateDefinition;

impl Parse for ServiceTemplateDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::ServiceTemplate);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "inputs" => {
                        parse_collection::<V::ParameterDefinition, V>(ctx, root, entry.1);
                    }
                    "outputs" => {
                        parse_collection::<V::ParameterDefinition, V>(ctx, root, entry.1);
                    }
                    "node_templates" => {
                        parse_collection::<V::NodeTemplateDefinition, V>(ctx, root, entry.1);
                    }
                    f => ctx.errors.push(Box::new(ParseError {
                        pos: Some(entry.0.pos()),
                        error: ParseErrorKind::UnknownField(f.to_string()),
                    })),
                });
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType("map"),
            }));
        }

        root
    }
}
