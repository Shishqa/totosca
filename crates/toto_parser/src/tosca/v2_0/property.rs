use toto_tosca::{Boolean, Entity, Relation};

use super::{parse_collection, value::Value, Reference};
use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct PropertyDefinition;

pub type PropertyAssignment = Value;

impl Parse for PropertyDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::Property);

        let mut has_type: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => {
                        has_type = true;
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Type);
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "metadata" => {
                        parse_collection::<String, V>(ctx, root, entry.1);
                    }
                    "status" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Status);
                    }
                    "default" => {
                        let t = V::Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Default);
                    }
                    "validation" => {
                        let t = V::Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Default);
                    }
                    "key_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::KeySchema);
                    }
                    "entry_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::EntrySchema);
                    }
                    "required" => {
                        let t = Boolean::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Required);
                    }
                    "value" => {
                        let t = V::Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Value);
                    }
                    "external-schema" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::ExternalSchema);
                    }
                    f => ctx.errors.push(Box::new(ParseError {
                        pos: Some(entry.0.pos()),
                        error: ParseErrorKind::UnknownField(f.to_string()),
                    })),
                });
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType(
                    "map (single-line notation is not supported)",
                ),
            }));
            return root;
        }

        if !has_type {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::MissingField("type"),
            }));
        }

        root
    }
}
