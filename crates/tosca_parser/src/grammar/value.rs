use super::{List, Map};
use crate::parse::{Error, Parse, ParseError};

pub type Integer = i64;
pub type Boolean = bool;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    // Primitive.
    String(String),
    Integer(Integer),
    // xxx: to implement Ord
    Float(String),
    Boolean(Boolean),
    Nil,

    // Collection.
    Map(Map<Value, Value>),
    List(List<Value>),
}

impl Parse for Value {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        match n.rc_ref().as_ref() {
            yaml_peg::Yaml::Null => Ok(Value::Nil),
            yaml_peg::Yaml::Int(_) => Integer::from_yaml(n).map(|i| Value::Integer(i)),
            yaml_peg::Yaml::Float(f) => Ok(Value::Float(f.clone())),
            yaml_peg::Yaml::Str(_) => String::from_yaml(n).map(|s| Value::String(s)),
            yaml_peg::Yaml::Bool(_) => Boolean::from_yaml(n).map(|b| Value::Boolean(b)),
            yaml_peg::Yaml::Seq(_) => List::<Value>::from_yaml(n).map(|l| Value::List(l)),
            yaml_peg::Yaml::Map(_) => Map::<Value, Value>::from_yaml(n).map(|m| Value::Map(m)),
            // TODO: handle anchors
            _ => unimplemented!(),
        }
    }
}

impl Parse for String {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        n.as_str()
            .map_err(|pos| {
                vec![Error {
                    pos: Some(pos),
                    error: ParseError::UnexpectedType("string"),
                }]
            })
            .map(|s| s.to_string())
    }
}

impl Parse for Integer {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        n.as_int().map_err(|pos| {
            vec![Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("integer"),
            }]
        })
    }
}

impl Parse for Boolean {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        n.as_bool().map_err(|pos| {
            vec![Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("boolean"),
            }]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn parse_string() {
        assert_eq!(
            parse::<Value>("test\n").unwrap(),
            Value::String("test".to_string())
        );
        assert_eq!(
            parse::<Value>("$escaped\n").unwrap(),
            Value::String("$escaped".to_string())
        );
        assert_eq!(
            parse::<Value>("0.0.1\n").unwrap(),
            Value::String("0.0.1".to_string())
        );
        assert_eq!(
            parse::<Value>("5     GiB\n").unwrap(),
            Value::String("5     GiB".to_string())
        );
        assert_eq!(
            parse::<Value>("'0.1'\n").unwrap(),
            Value::String("0.1".to_string())
        );
        assert_eq!(
            parse::<Value>("''\n").unwrap(),
            Value::String("".to_string())
        );
        assert_eq!(
            parse::<Value>("😏\n").unwrap(),
            Value::String("😏".to_string())
        );
    }

    #[test]
    fn parse_integer() {
        assert_eq!(parse::<Value>("42\n").unwrap(), Value::Integer(42));
        assert_eq!(parse::<Value>("-42\n").unwrap(), Value::Integer(-42));
        assert_eq!(parse::<Value>("0\n").unwrap(), Value::Integer(0));
        assert_eq!(parse::<Value>("-0\n").unwrap(), Value::Integer(0));
        assert_eq!(parse::<Value>("0x10\n").unwrap(), Value::Integer(16));
        assert_eq!(parse::<Value>("0o10\n").unwrap(), Value::Integer(8));
    }

    #[test]
    fn parse_float() {
        assert_eq!(
            parse::<Value>("0.1\n").unwrap(),
            Value::Float("0.1".to_string())
        );
        assert_eq!(
            parse::<Value>("2e4\n").unwrap(),
            Value::Float("2e4".to_string())
        );
        assert_eq!(
            parse::<Value>(".inf\n").unwrap(),
            Value::Float("inf".to_string())
        );
        assert_eq!(
            parse::<Value>(".nan\n").unwrap(),
            Value::Float("NaN".to_string())
        );
    }

    #[test]
    fn parse_bool() {
        assert_eq!(parse::<Value>("true\n").unwrap(), Value::Boolean(true));
        assert_eq!(parse::<Value>("True\n").unwrap(), Value::Boolean(true));
        assert_eq!(parse::<Value>("false\n").unwrap(), Value::Boolean(false));
        assert_eq!(parse::<Value>("False\n").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn parse_nil() {
        assert_eq!(parse::<Value>("null\n").unwrap(), Value::Nil);
    }

    #[test]
    fn parse_list() {
        const LONG_LIST: &str = "
- a
- 1.2
- 42\n";

        const SHORT_LIST: &str = "[a, 1.2, 42]";

        let check: Value = Value::List(List::from_iter([
            Value::String("a".to_string()),
            Value::Float("1.2".to_string()),
            Value::Integer(42),
        ]));

        assert_eq!(parse::<Value>(LONG_LIST).unwrap(), check);
        assert_eq!(parse::<Value>(SHORT_LIST).unwrap(), check);
    }

    #[test]
    fn parse_map() {
        const LONG_MAP: &str = "
str_key: a
42: 1.2
{
    map: key
}: 42\n";

        const SHORT_MAP: &str = "{ str_key: a, 42: 1.2, { map: key }: 42 }";

        let check: Value = Value::Map(Map::from_iter([
            (
                Value::String("str_key".to_string()),
                Value::String("a".to_string()),
            ),
            (Value::Integer(42), Value::Float("1.2".to_string())),
            (
                Value::Map(Map::from_iter([(
                    Value::String("map".to_string()),
                    Value::String("key".to_string()),
                )])),
                Value::Integer(42),
            ),
        ]));

        assert_eq!(parse::<Value>(LONG_MAP).unwrap(), check);
        assert_eq!(parse::<Value>(SHORT_MAP).unwrap(), check);
    }
}
