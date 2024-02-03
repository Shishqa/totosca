use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

use super::Reference;

#[derive(Debug)]
pub struct SchemaDefinition;

impl Parse for SchemaDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::Schema);

        let mut has_type: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => {
                        has_type = true;
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Type);
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "validation" => {
                        let t = V::Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Validation);
                    }
                    "key_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::KeySchema);
                    }
                    "entry_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::EntrySchema);
                    }
                    f => ctx.errors.push(Box::new(ParseError {
                        pos: Some(entry.0.pos()),
                        error: ParseErrorKind::UnknownField(f.to_string()),
                    })),
                });
        } else if n.as_str().is_ok() {
            has_type = true;
            let t = String::parse::<V>(ctx, n);
            ctx.graph.add_edge(root, t, Relation::Type);
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType("map or string"),
            }));
            return root;
        }

        if !has_type {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::MissingField("type"),
            }));
        }

        root
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::parse::{parse, ParseError};
//
//     #[test]
//     fn parse_schema_definition() {
//         const SCHEMA_DEFINITION: &str = "
// type: some.types.A
// description: a description
// validation:
// - $and:
//     $greater_or_equal: [ $value, 0 ]
// entry_schema: short.notation.Type
// key_schema:
//   type: long.notation.Type
//   description: key description\n";
//
//         parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap();
//     }
//
//     #[test]
//     fn fail_if_missing_type() {
//         const SCHEMA_DEFINITION: &str = "
// # type: missing
// description: a description
// validation:
// - $and:
//     $greater_or_equal: [ $value, 0 ]
// entry_schema: short.notation.Type
// key_schema:
//   # type: missing
//   description: key description\n";
//
//         let err = parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err();
//         assert!(err.len() == 2);
//         assert!(err
//             .iter()
//             .all(|e| e.error == ParseErrorKind::MissingField("type")));
//     }
//
//     #[test]
//     fn fail_if_unknown_field() {
//         const SCHEMA_DEFINITION: &str = "
// type: some.types.A
// description: a description
// typo:
//   some:
//     info: should fail
// validation:
// - $and:
//     $greater_or_equal: [ $value, 0 ]
// entry_schema: short.notation.Type
// key_schema:
//   # type: missing
//   description: key description\n";
//
//         let err = parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err();
//         assert!(err.len() == 2);
//         assert!(err
//             .iter()
//             .any(|e| e.error == ParseErrorKind::UnknownField("typo".to_string())));
//         // should still report other errors.
//         assert!(err
//             .iter()
//             .any(|e| e.error == ParseErrorKind::MissingField("type")));
//     }
//
//     #[test]
//     fn fail_if_wrong_type() {
//         const SCHEMA_DEFINITION: &str = "[ a, b, c ]";
//
//         let err = dbg!(parse::<SchemaDefinition>(SCHEMA_DEFINITION).unwrap_err());
//         assert!(err.len() == 1);
//         assert!(err[0].error == ParseErrorKind::UnexpectedType("map or string"));
//     }
// }
