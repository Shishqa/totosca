use super::{Map, PropertyDefinitions, SchemaDefinition, Value};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct DataType {
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub metadata: Map<String, String>,
    pub version: Option<String>,

    pub validation: Option<Value>,
    pub properties: PropertyDefinitions,
    pub key_schema: Option<SchemaDefinition>,
    pub entry_schema: Option<SchemaDefinition>,
}

pub type DataTypes = Map<String, DataType>;

impl Parse for DataType {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut derived_from: Option<String> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;
        let mut version: Option<String> = None;

        let mut validation: Option<Value> = None;
        let mut properties: Option<PropertyDefinitions> = None;
        let mut key_schema: Option<SchemaDefinition> = None;
        let mut entry_schema: Option<SchemaDefinition> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "derived_from" => get_field(entry.1, &mut derived_from, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "metadata" => get_field(entry.1, &mut metadata, &mut errors),
                    "version" => get_field(entry.1, &mut version, &mut errors),
                    "validation" => get_field(entry.1, &mut validation, &mut errors),
                    "properties" => get_field(entry.1, &mut properties, &mut errors),
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

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                derived_from,
                description,
                metadata: metadata.unwrap_or(Map::new()),
                version,
                validation,
                properties: properties.unwrap_or(Map::new()),
                key_schema,
                entry_schema,
            })
        }
    }
}
