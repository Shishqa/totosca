use super::Value;
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct SchemaDefinition {
    pub type_ref: String,
    pub description: Option<String>,
    pub validation: Option<Value>,
    pub key_schema: Option<Box<SchemaDefinition>>,
    pub entry_schema: Option<Box<SchemaDefinition>>,
}

impl Parse for SchemaDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut type_ref: Option<String> = None;
        let mut description: Option<String> = None;
        let mut validation: Option<Value> = None;
        let mut key_schema: Option<Box<SchemaDefinition>> = None;
        let mut entry_schema: Option<Box<SchemaDefinition>> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => get_field(entry.1, &mut type_ref, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "validation" => get_field(entry.1, &mut validation, &mut errors),
                    "key_schema" => get_field(entry.1, &mut key_schema, &mut errors),
                    "entry_schema" => get_field(entry.1, &mut entry_schema, &mut errors),
                    f => errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(s) = n.as_str() {
            type_ref = Some(s.to_string());
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
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
                key_schema,
                entry_schema,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{parse, ParseError};

    #[test]
    fn parse_schema_definition() {
        const SCHEMA_DEFINITION: &str = "
type: some.types.A
description: a description
validation:
- $and:
    $greater_or_equal: [ $value, 0 ]
entry_schema: short.notation.Type
key_schema:
  type: long.notation.Type
  description: key description\n";

        parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap();
    }

    #[test]
    fn fail_if_missing_type() {
        const SCHEMA_DEFINITION: &str = "
# type: missing
description: a description
validation:
- $and:
    $greater_or_equal: [ $value, 0 ]
entry_schema: short.notation.Type
key_schema:
  # type: missing
  description: key description\n";

        let err = parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err();
        assert!(err.len() == 2);
        assert!(err
            .iter()
            .all(|e| e.error == ParseError::MissingField("type")));
    }

    #[test]
    fn fail_if_unknown_field() {
        const SCHEMA_DEFINITION: &str = "
type: some.types.A
description: a description
typo:
  some:
    info: should fail
validation:
- $and:
    $greater_or_equal: [ $value, 0 ]
entry_schema: short.notation.Type
key_schema:
  # type: missing
  description: key description\n";

        let err = parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err();
        assert!(err.len() == 2);
        assert!(err
            .iter()
            .any(|e| e.error == ParseError::UnknownField("typo".to_string())));
        // should still report other errors.
        assert!(err
            .iter()
            .any(|e| e.error == ParseError::MissingField("type")));
    }

    #[test]
    fn fail_if_wrong_type() {
        const SCHEMA_DEFINITION: &str = "[ a, b, c ]";

        let err = dbg!(parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err());
        assert!(err.len() == 1);
        assert!(err[0].error == ParseError::UnexpectedType("map or string"));
    }
}
