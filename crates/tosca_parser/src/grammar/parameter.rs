use super::{Boolean, List, Map, SchemaDefinition, Value};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct ParameterDefinition {
    pub type_ref: Option<String>,
    pub description: Option<String>,
    pub required: Option<Boolean>,
    pub default: Option<Value>,
    pub value: Option<Value>,
    pub mapping: Option<List<Value>>,
    pub status: Option<String>,
    pub validation: Option<Value>,
    pub key_schema: Option<SchemaDefinition>,
    pub entry_schema: Option<SchemaDefinition>,
    pub external_schema: Option<String>,
    pub metadata: Map<String, String>,
}

pub type ParameterDefinitions = Map<String, ParameterDefinition>;

impl Parse for ParameterDefinition {
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

        let mut required: Option<Boolean> = None;
        let mut value: Option<Value> = None;
        let mut mapping: Option<List<Value>> = None;
        let mut external_schema: Option<String> = None;

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
                    "required" => get_field(entry.1, &mut required, &mut errors),
                    "value" => get_field(entry.1, &mut value, &mut errors),
                    "mapping" => get_field(entry.1, &mut mapping, &mut errors),
                    "external-schema" => get_field(entry.1, &mut external_schema, &mut errors),
                    f => errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map (single-line notation is not supported)"),
            });
            return Err(errors);
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                type_ref,
                description,
                validation,
                metadata: metadata.unwrap_or(Map::new()),
                status,
                default,
                key_schema,
                entry_schema,
                required,
                value,
                mapping,
                external_schema,
            })
        }
    }
}
