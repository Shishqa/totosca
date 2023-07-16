use super::{Integer, List, Value};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct RequirementDefinition {
    pub name: String,
    pub description: Option<String>,
    pub node: String,
    pub count_range: Option<List<Value>>,
}

pub type RequirementDefinitions = List<RequirementDefinition>;

#[derive(Debug)]
pub struct RequirementAssignment {
    pub name: String,
    pub node: Option<String>,
    pub count: Option<Integer>,
}

pub type RequirementAssignments = List<RequirementAssignment>;

impl Parse for RequirementDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let name: Option<String>;

        let mut description: Option<String> = None;
        let mut node: Option<String> = None;
        let mut count_range: Option<List<Value>> = None;

        if let Ok(map) = n.as_map() {
            if map.len() != 1 {
                errors.push(Error {
                    pos: Some(n.pos()),
                    error: ParseError::Custom("should have only one key"),
                });
                return Err(errors);
            }

            let (key, value) = map.iter().next().unwrap();

            name = String::from_yaml(key).map_err(|e| errors.extend(e)).ok();

            if let Ok(map) = value.as_map() {
                map.iter()
                    .for_each(|entry| match entry.0.as_str().unwrap() {
                        "description" => get_field(entry.1, &mut description, &mut errors),
                        "node" => get_field(entry.1, &mut node, &mut errors),
                        "count_range" => get_field(entry.1, &mut count_range, &mut errors),
                        f => errors.push(Error {
                            pos: Some(entry.0.pos()),
                            error: ParseError::UnknownField(f.to_string()),
                        }),
                    });
            } else if let Ok(_) = value.as_str() {
                node = String::from_yaml(value).map_err(|e| errors.extend(e)).ok();
            } else {
                errors.push(Error {
                    pos: Some(n.pos()),
                    error: ParseError::UnexpectedType("map or string"),
                });
                return Err(errors);
            }
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return Err(errors);
        }

        if node.is_none() {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("node"),
            });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                name: name.unwrap(),
                description,
                node: node.unwrap(),
                count_range,
            })
        }
    }
}

impl Parse for RequirementAssignment {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let name: Option<String>;

        let mut node: Option<String> = None;
        let mut count: Option<Integer> = None;

        if let Ok(map) = n.as_map() {
            if map.len() != 1 {
                errors.push(Error {
                    pos: Some(n.pos()),
                    error: ParseError::Custom("should have only one key"),
                });
                return Err(errors);
            }

            let (key, value) = map.iter().next().unwrap();

            name = String::from_yaml(key).map_err(|e| errors.extend(e)).ok();

            if let Ok(map) = value.as_map() {
                map.iter()
                    .for_each(|entry| match entry.0.as_str().unwrap() {
                        "node" => get_field(entry.1, &mut node, &mut errors),
                        "count" => get_field(entry.1, &mut count, &mut errors),
                        f => errors.push(Error {
                            pos: Some(entry.0.pos()),
                            error: ParseError::UnknownField(f.to_string()),
                        }),
                    });
            } else if let Ok(_) = value.as_str() {
                node = String::from_yaml(value).map_err(|e| errors.extend(e)).ok();
            } else {
                errors.push(Error {
                    pos: Some(n.pos()),
                    error: ParseError::UnexpectedType("map or string"),
                });
                return Err(errors);
            }
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return Err(errors);
        }

        if node.is_none() {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("node"),
            });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                name: name.unwrap(),
                node,
                count,
            })
        }
    }
}
