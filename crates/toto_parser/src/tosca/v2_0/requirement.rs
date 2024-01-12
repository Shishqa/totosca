use toto_tosca::{Entity, Integer, Relation};

use crate::{
    parse::{Context, Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

use super::{value::Value, List, Reference};

#[derive(Debug)]
pub struct RequirementDefinition;

#[derive(Debug)]
pub struct RequirementAssignment;

pub fn parse_keyed_list_collection<P: Parse, V: ToscaDefinitionsVersion>(
    ctx: &mut Context,
    root: GraphHandle,
    n: &yaml_peg::NodeRc,
) {
    n.as_seq()
        .map_err(|pos| {
            ctx.errors.push(Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("list"),
            })
        })
        .map(|seq| {
            for (idx, item) in seq.iter().enumerate() {
                if let Ok(map) = item.as_map() {
                    if map.len() != 1 {
                        ctx.errors.push(Error {
                            pos: Some(item.pos()),
                            error: ParseError::Custom("should have only one key".to_string()),
                        });
                        continue;
                    }

                    let (key, value) = map.iter().next().unwrap();

                    let name = String::from_yaml(key)
                        .map_err(|err| ctx.errors.push(err))
                        .unwrap_or_default();

                    let elem = P::parse::<V>(ctx, &value);

                    ctx.graph.add_edge(root, elem, Relation::Subdef(name));
                    ctx.graph
                        .add_edge(root, elem, Relation::ListValue(idx as u64));
                } else {
                    ctx.errors.push(Error {
                        pos: Some(item.pos()),
                        error: ParseError::UnexpectedType("map"),
                    });
                    continue;
                }
            }
        });
}

impl Parse for RequirementDefinition {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Requirement);

        let mut has_node: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "node" => {
                        has_node = true;
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Node);
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "count_range" => {
                        let t = List::<V::Value>::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::CountRange);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(r) = n.as_str() {
            has_node = true;
            let t = ctx.graph.add_node(Entity::Ref(r.to_string()));
            ctx.graph.add_edge(root, t, Relation::Node);
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
            });
            return root;
        }

        if !has_node {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("node"),
            });
        }

        root
    }
}

impl Parse for RequirementAssignment {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Requirement);

        let mut has_node: bool = false;
        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "node" => {
                        has_node = true;
                        let t = Reference::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Node);
                    }
                    "count" => {
                        let t = Integer::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Count);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(r) = n.as_str() {
            has_node = true;
            let t = ctx.graph.add_node(Entity::Ref(r.to_string()));
            ctx.graph.add_edge(root, t, Relation::Node);
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
            });
            return root;
        }

        if !has_node {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("node"),
            });
        }

        root
    }
}
