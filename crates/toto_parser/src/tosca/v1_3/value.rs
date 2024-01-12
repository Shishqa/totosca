use std::collections::HashSet;

use once_cell::sync::Lazy;
use toto_tosca::{Boolean, Entity, Float, Integer, Relation};

use crate::{
    parse::{Context, GraphHandle},
    tosca::{
        v2_0::{parse_list, List, Map},
        Parse, ToscaDefinitionsVersion,
    },
};

pub struct Value;

impl Parse for Value {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        static FUNCS: Lazy<HashSet<&'static str>> =
            Lazy::new(|| HashSet::from(["get_input", "get_property", "get_attribute"]));

        if let Ok(map) = n.as_map() {
            if map.len() == 1 {
                let elem = map.iter().next().unwrap();
                if let Ok(s) = elem.0.as_str() {
                    if FUNCS.contains(s) {
                        let root = ctx.graph.add_node(Entity::FunctionCall);
                        let r = ctx.graph.add_node(Entity::Ref(s.to_string()));
                        ctx.graph.add_edge(root, r, Relation::Function);
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
