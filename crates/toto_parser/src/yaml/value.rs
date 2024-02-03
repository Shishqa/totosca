use toto_tosca::{Boolean, Float, Integer};

use crate::parse::{ParseError, ParseErrorKind};

use super::FromYaml;

impl FromYaml for String {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, ParseError> {
        n.as_str()
            .map_err(|pos| ParseError {
                pos: Some(pos),
                error: ParseErrorKind::UnexpectedType("string"),
            })
            .map(|s| s.to_string())
    }
}

impl FromYaml for Integer {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, ParseError> {
        n.as_int().map_err(|pos| ParseError {
            pos: Some(pos),
            error: ParseErrorKind::UnexpectedType("integer"),
        })
    }
}

impl FromYaml for Boolean {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, ParseError> {
        n.as_bool().map_err(|pos| ParseError {
            pos: Some(pos),
            error: ParseErrorKind::UnexpectedType("boolean"),
        })
    }
}

impl FromYaml for Float {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, ParseError> {
        n.as_float()
            .map_err(|pos| ParseError {
                pos: Some(pos),
                error: ParseErrorKind::UnexpectedType("boolean"),
            })
            .map(|v| v.into())
    }
}
