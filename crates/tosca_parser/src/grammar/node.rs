use super::{
    AttributeAssignments, AttributeDefinitions, Map, PropertyAssignments, PropertyDefinitions,
    RequirementAssignments, RequirementDefinitions,
};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct NodeType {
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub metadata: Map<String, String>,
    pub version: Option<String>,

    pub properties: PropertyDefinitions,
    pub attributes: AttributeDefinitions,
    pub requirements: RequirementDefinitions,
}

pub type NodeTypes = Map<String, NodeType>;

#[derive(Debug)]
pub struct NodeTemplate {
    pub type_ref: String,
    pub description: Option<String>,
    pub metadata: Map<String, String>,

    pub properties: PropertyAssignments,
    pub attributes: AttributeAssignments,
    pub requirements: RequirementAssignments,
}

pub type NodeTemplates = Map<String, NodeTemplate>;

impl Parse for NodeType {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut derived_from: Option<String> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;
        let mut version: Option<String> = None;

        let mut properties: Option<PropertyDefinitions> = None;
        let mut attributes: Option<AttributeDefinitions> = None;
        let mut requirements: Option<RequirementDefinitions> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "derived_from" => get_field(entry.1, &mut derived_from, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "metadata" => get_field(entry.1, &mut metadata, &mut errors),
                    "version" => get_field(entry.1, &mut version, &mut errors),
                    "properties" => get_field(entry.1, &mut properties, &mut errors),
                    "attributes" => get_field(entry.1, &mut attributes, &mut errors),
                    "requirements" => get_field(entry.1, &mut requirements, &mut errors),
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
                properties: properties.unwrap_or(Map::new()),
                attributes: attributes.unwrap_or(Map::new()),
                requirements: requirements.unwrap_or(RequirementDefinitions::new()),
            })
        }
    }
}

impl Parse for NodeTemplate {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut type_ref: Option<String> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;

        let mut properties: Option<PropertyAssignments> = None;
        let mut attributes: Option<AttributeAssignments> = None;
        let mut requirements: Option<RequirementAssignments> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => get_field(entry.1, &mut type_ref, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "metadata" => get_field(entry.1, &mut metadata, &mut errors),
                    "properties" => get_field(entry.1, &mut properties, &mut errors),
                    "attributes" => get_field(entry.1, &mut attributes, &mut errors),
                    "requirements" => get_field(entry.1, &mut requirements, &mut errors),
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
                metadata: metadata.unwrap_or(Map::new()),
                properties: properties.unwrap_or(Map::new()),
                attributes: attributes.unwrap_or(Map::new()),
                requirements: requirements.unwrap_or(RequirementAssignments::new()),
            })
        }
    }
}
