use toto_tosca::{Boolean, Entity, Relation};

use super::{parse_collection, value::Value, List, SchemaDefinition};
use crate::{
    parse::{Context, Error, GraphHandle, ParseError},
    tosca::{FromYaml, Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ParameterDefinition;

impl Parse for ParameterDefinition {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Parameter);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => {
                        String::from_yaml(entry.1)
                            .map_err(|err| ctx.errors.push(err))
                            .map(|r| {
                                let t = ctx.graph.add_node(Entity::Ref(r));
                                ctx.graph.add_edge(root, t, Relation::Type);
                            });
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "metadata" => {
                        parse_collection::<String, V>(ctx, root, entry.1);
                    }
                    "status" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Status);
                    }
                    "default" => {
                        let t = Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Default);
                    }
                    "validation" => {
                        let t = Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Default);
                    }
                    "key_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::KeySchema);
                    }
                    "entry_schema" => {
                        let t = V::SchemaDefinition::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::EntrySchema);
                    }
                    "required" => {
                        let t = Boolean::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Required);
                    }
                    "value" => {
                        let t = Value::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Value);
                    }
                    "mapping" => {
                        let t = List::<String>::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Mapping);
                    }
                    "external-schema" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::ExternalSchema);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map (single-line notation is not supported)"),
            });
            return root;
        }

        root
    }
}
