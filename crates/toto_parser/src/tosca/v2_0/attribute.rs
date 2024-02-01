use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

use super::{parse_collection, Reference, Value};

#[derive(Debug)]
pub struct AttributeDefinition;

pub type AttributeAssignment = Value;

impl Parse for AttributeDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::Attribute);

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
