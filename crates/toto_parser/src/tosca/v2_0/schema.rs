use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{add_error, ParseError, ParseLoc, StaticSchema},
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        ToscaDefinitionsVersion,
    },
};

use super::Reference;

#[derive(Debug)]
pub struct SchemaDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(V, E, R)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for SchemaDefinition<E, R, V> {
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> StaticSchema<E, R> for SchemaDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::Schema;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "type" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::DataType.into());
            ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, *v, ParseLoc.into());
            has_url = Some(t);

            has_type = true;
            let t = Reference::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Type);
        },
        "description" => |r, n, ast|{
            let t = String::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Description);
        },
        "validation" => |r, n, ast|{
            let t = V::Value::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::Validation);
        },
        "key_schema" => |r, n, ast|{
            let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::KeySchema);
        },
        "entry_schema" => |r, n, ast|{
            let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
            ctx.graph.add_edge(root, t, Relation::EntrySchema);
        },
    };
}

impl<E, R, V> toto_ast::Parse<E, R> for SchemaDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        match t {
            toto_yaml::Entity::Map => Self::parse_schema(self.0, ast),
            toto_yaml::Entity::Str(ref_type) => {
                let root = ast.add_node(toto_tosca::Entity::Schema.into());
                ast.add_edge(root, self.0, ParseLoc.into());

                let t = ast.add_node(toto_tosca::Entity::Ref { name: ref_type }.into());
                ast.add_edge(root, t, toto_tosca::Relation::HasType.into());
                ast.add_edge(t, self.0, ParseLoc.into());

                root
            }
            _ => {
                add_error(self.0, ast, ParseError::UnexpectedType("map or string"));
                self.0
            }
        }
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
