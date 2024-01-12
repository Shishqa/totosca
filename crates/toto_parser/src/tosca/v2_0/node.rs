use toto_tosca::{Entity, Relation};

use crate::{
    parse::{Error, ParseError},
    tosca::{FromYaml, Parse, ToscaDefinitionsVersion},
};

use super::{
    parse_collection, parse_keyed_list_collection, AttributeAssignment, AttributeDefinition,
    PropertyAssignment, PropertyDefinition, RequirementDefinition,
};

#[derive(Debug)]
pub struct NodeType;

#[derive(Debug)]
pub struct NodeTemplate;

impl Parse for NodeType {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut crate::parse::Context,
        n: &yaml_peg::NodeRc,
    ) -> crate::parse::GraphHandle {
        let root = ctx.graph.add_node(Entity::NodeType);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "derived_from" => {
                        String::from_yaml(entry.1)
                            .map_err(|err| ctx.errors.push(err))
                            .map(|r| {
                                let t = ctx.graph.add_node(Entity::Ref(r));
                                ctx.graph.add_edge(root, t, Relation::DerivedFrom);
                            });
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
                    "properties" => {
                        parse_collection::<V::PropertyDefinition, V>(ctx, root, entry.1);
                    }
                    "attributes" => {
                        parse_collection::<V::AttributeDefinition, V>(ctx, root, entry.1);
                    }
                    "requirements" => {
                        parse_keyed_list_collection::<V::RequirementDefinition, V>(
                            ctx, root, entry.1,
                        );
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
        }

        root
    }
}

impl Parse for NodeTemplate {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut crate::parse::Context,
        n: &yaml_peg::NodeRc,
    ) -> crate::parse::GraphHandle {
        let root = ctx.graph.add_node(Entity::NodeType);

        let mut has_type: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "type" => {
                        has_type = true;
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
                    "properties" => {
                        parse_collection::<V::PropertyAssignment, V>(ctx, root, entry.1);
                    }
                    "attributes" => {
                        parse_collection::<V::AttributeAssignment, V>(ctx, root, entry.1);
                    }
                    "requirements" => {
                        parse_keyed_list_collection::<V::RequirementDefinition, V>(
                            ctx, root, entry.1,
                        );
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
        }

        if !has_type {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("type"),
            });
        }

        root
    }
}
