use petgraph::Direction::Incoming;
use toto_tosca::{Boolean, Entity, Float, Integer, Relation};

use crate::tosca::{
    v2_0::{parse_list, List, Map},
    Parse, ToscaDefinitionsVersion,
};

pub struct Value;

impl Parse for Value {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        if let Ok(map) = n.as_map() {
            if map.len() == 1 {
                let elem = map.iter().next().unwrap();
                if let Ok(s) = elem.0.as_str() {
                    let available_func = ctx
                        .graph
                        .node_indices()
                        .filter(|i| matches!(ctx.graph[*i], Entity::Function))
                        .find(|i| {
                            ctx.graph
                                .edges_directed(*i, Incoming)
                                .any(|e| match e.weight() {
                                    Relation::Subdef(name) => name == s,
                                    _ => false,
                                })
                        });

                    if let Some(func) = available_func {
                        let root = ctx.graph.add_node(Entity::FunctionCall);
                        ctx.graph.add_edge(root, func, Relation::Function);
                        parse_list::<V::Value, V>(ctx, root, elem.1);

                        return root;
                    }
                }
            }
        }

        match n.rc_ref().as_ref() {
            yaml_peg::Yaml::Null => ctx.graph.add_node(Entity::Nil),
            yaml_peg::Yaml::Int(_) => Integer::parse::<V>(ctx, n),
            yaml_peg::Yaml::Float(_) => Float::parse::<V>(ctx, n),
            yaml_peg::Yaml::Str(_) => String::parse::<V>(ctx, n),
            yaml_peg::Yaml::Bool(_) => Boolean::parse::<V>(ctx, n),
            yaml_peg::Yaml::Seq(_) => List::<Value>::parse::<V>(ctx, n),
            yaml_peg::Yaml::Map(_) => Map::<Value, Value>::parse::<V>(ctx, n),
            // TODO: handle anchors
            _ => unimplemented!(),
        }
    }
}
