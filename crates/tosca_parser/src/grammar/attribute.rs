use super::{Map, SchemaDefinition, Value};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct AttributeDefinition {
    pub type_ref: String,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub status: Option<String>,
    pub validation: Option<Value>,
    pub key_schema: Option<SchemaDefinition>,
    pub entry_schema: Option<SchemaDefinition>,
    pub metadata: Map<String, String>,
}

pub type AttributeDefinitions = Map<String, AttributeDefinition>;

pub type AttributeAssignment = Value;

pub type AttributeAssignments = Map<String, AttributeAssignment>;

impl Parse for AttributeDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut type_ref: Option<String> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;

        let mut default: Option<Value> = None;
        let mut status: Option<String> = None;
        let mut validation: Option<Value> = None;
        let mut key_schema: Option<SchemaDefinition> = None;
        let mut entry_schema: Option<SchemaDefinition> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => get_field(entry.1, &mut type_ref, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "metadata" => get_field(entry.1, &mut metadata, &mut errors),
                    "status" => get_field(entry.1, &mut status, &mut errors),
                    "default" => get_field(entry.1, &mut default, &mut errors),
                    "validation" => get_field(entry.1, &mut validation, &mut errors),
                    "key_schema" => get_field(entry.1, &mut key_schema, &mut errors),
                    "entry_schema" => get_field(entry.1, &mut entry_schema, &mut errors),
                    f => errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return Err(errors);
        }

        if type_ref.is_none() {
            errors.push(Error {
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
