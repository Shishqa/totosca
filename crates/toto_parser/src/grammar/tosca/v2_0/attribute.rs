use toto_tosca::{Entity, Relation};

use crate::parse::{Context, Error, FromYaml, GraphHandle, Parse, ParseError};

use super::{parse_collection, value::Value, SchemaDefinition};

#[derive(Debug)]
pub struct AttributeDefinition;

pub type AttributeAssignment = Value;

impl Parse for AttributeDefinition {
    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Attribute);

        let mut has_type: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter().map(|entry| match entry.0.as_str().unwrap() {
                "type" => {
                    has_type = true;
                    String::from_yaml(entry.1)
                        .map_err(|err| ctx.errors.push(err))
                        .map(|r| {
                            let t = ctx.graph.add_node(Entity::Ref(r));
                            ctx.graph.add_edge(root, t, Relation::Type);
                        });
                }
                "description" => {
                    let t = String::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::Description);
                }
                "metadata" => {
                    parse_collection::<String>(ctx, root, entry.1);
                }
                "status" => {
                    let t = String::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::Status);
                }
                "default" => {
                    let t = Value::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::Default);
                }
                "validation" => {
                    let t = Value::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::Default);
                }
                "key_schema" => {
                    let t = SchemaDefinition::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::KeySchema);
                }
                "entry_schema" => {
                    let t = SchemaDefinition::parse(ctx, entry.1);
                    ctx.graph.add_edge(root, t, Relation::EntrySchema);
                }
                f => ctx.errors.push(Error {
                    pos: Some(entry.0.pos()),
                    error: ParseError::UnknownField(f.to_string()),
                }),
            });
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return root;
        }

        if !has_type {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("type"),
            });
        }

        root
    }
}
