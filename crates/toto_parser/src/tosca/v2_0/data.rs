use toto_tosca::{Entity, Relation};

use super::{parse_collection, value::Value, Reference};
use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct DataType;

impl Parse for DataType {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::DataType);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "derived_from" => {
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::DerivedFrom);
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "metadata" => {
                        parse_collection::<String, V>(ctx, root, entry.1);
                    }
                    "version" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Version);
                    }
                    "validation" => {
                        let t = Value::parse::<V>(ctx, entry.1);
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
                    "properties" => {
                        parse_collection::<V::PropertyDefinition, V>(ctx, root, entry.1);
                    }
                    f => ctx.errors.push(Box::new(ParseError {
                        pos: Some(entry.0.pos()),
                        error: ParseErrorKind::UnknownField(f.to_string()),
                    })),
                });
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType("map"),
            }));
            return root;
        }

        root
    }
}
