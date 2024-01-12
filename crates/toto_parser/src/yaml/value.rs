use toto_tosca::{Boolean, Float, Integer};

use crate::parse::{Error, ParseError};

use super::FromYaml;

impl FromYaml for String {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error> {
        n.as_str()
            .map_err(|pos| Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("string"),
            })
            .map(|s| s.to_string())
    }
}

impl FromYaml for Integer {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error> {
        n.as_int().map_err(|pos| Error {
            pos: Some(pos),
            error: ParseError::UnexpectedType("integer"),
        })
    }
}

impl FromYaml for Boolean {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error> {
        n.as_bool().map_err(|pos| Error {
            pos: Some(pos),
            error: ParseError::UnexpectedType("boolean"),
        })
    }
}

impl FromYaml for Float {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error> {
        n.as_float()
            .map_err(|pos| Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("boolean"),
            })
            .map(|v| v.into())
    }
}
