use petgraph::stable_graph::NodeIndex;

use crate::{
    grammar::tosca::v2_0::{Map, Value},
    parse::{Context, Error, Parse, ParseError},
};

pub struct AttributeDefinition;

pub type AttributeDefinitions = Map<String, AttributeDefinition>;

pub type AttributeAssignment = Value;

pub type AttributeAssignments = Map<String, AttributeAssignment>;

impl Parse for AttributeDefinition {
    fn from_yaml(ctx: &mut Context, n: &yaml_peg::NodeRc) -> anyhow::Result<NodeIndex> {
        let mut type_ref: Option<String> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;

        let mut default: Option<Value> = None;
        let mut status: Option<String> = None;
        let mut validation: Option<Value> = None;
        let mut key_schema: Option<SchemaDefinition> = None;
        let mut entry_schema: Option<SchemaDefinition> = None;

        let n = ctx
            .graph
            .add_node(toto_tosca::Entity::Attribute { description: None });

        if let Ok(map) = n.as_map() {
            map.iter().map(|entry| match entry.0.as_str().unwrap() {
                "type" => String::from_yaml(&mut ctx, entry.1),
                "description" => String::from_yaml(&mut ctx, entry.1),
                "metadata" => String::from_yaml(&mut ctx, entry.1),
                "status" => String::from_yaml(&mut ctx, entry.1),
                "default" => String::from_yaml(&mut ctx, entry.1),
                "validation" => String::from_yaml(&mut ctx, entry.1),
                "key_schema" => String::from_yaml(&mut ctx, entry.1),
                "entry_schema" => String::from_yaml(&mut ctx, entry.1),
                f => ctx.errors.push(Error {
                    pos: Some(entry.0.pos()),
                    error: ParseError::UnknownField(f.to_string()),
                }),
            }).;
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return anyhow::anyhow!("");
        }

        if type_ref.is_none() {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("type"),
            });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                type_ref: type_ref.unwrap(),
                description,
                validation,
                metadata: metadata.unwrap_or(Map::new()),
                status,
                default,
                key_schema,
                entry_schema,
            })
        }
    }
}
