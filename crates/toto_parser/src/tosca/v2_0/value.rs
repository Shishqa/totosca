use toto_tosca::{Boolean, Entity, Float, Integer, Relation};

use super::{parse_list, List, Map};
use crate::{
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
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
                    if s.chars().nth(0).is_some_and(|c| c == '$')
                        && s.chars().nth(1).is_some_and(|c| c != '$')
                    {
                        let root = ctx.graph.add_node(Entity::FunctionCall);
                        let r = ctx.graph.add_node(Entity::Ref(s.to_string()));
                        ctx.graph.add_edge(root, r, Relation::Type);
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

impl Parse for Float {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        ctx.graph.add_node(Entity::Float(
            Float::from_yaml(n)
                .map_err(|err| ctx.errors.push(Box::new(err)))
                .unwrap_or_default(),
        ))
    }
}

impl Parse for String {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        ctx.graph.add_node(Entity::String(
            String::from_yaml(n)
                .map_err(|err| ctx.errors.push(Box::new(err)))
                .unwrap_or_default(),
        ))
    }
}

impl Parse for Integer {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        ctx.graph.add_node(Entity::Integer(
            Integer::from_yaml(n)
                .map_err(|err| ctx.errors.push(Box::new(err)))
                .unwrap_or_default(),
        ))
    }
}

impl Parse for Boolean {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        ctx.graph.add_node(Entity::Boolean(
            Boolean::from_yaml(n)
                .map_err(|err| ctx.errors.push(Box::new(err)))
                .unwrap_or_default(),
        ))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{collections::BTreeSet, fmt::Debug};
//
//     use petgraph::{
//         data::{Element, FromElements},
//         stable_graph::{NodeIndex, StableDiGraph, StableGraph},
//         visit::IntoNodeReferences,
//     };
//     use toto_tosca::{Entity, Relation};
//
//     use super::*;
//     use crate::parse::parse;
//
//     fn graph_repr<N, E, Ty, Ix>(
//         g: &StableGraph<N, E, Ty, Ix>,
//     ) -> (BTreeSet<&N>, BTreeSet<(NodeIndex<Ix>, NodeIndex<Ix>, &E)>)
//     where
//         N: PartialEq + Debug + Ord,
//         E: PartialEq + Debug + Ord,
//         Ty: petgraph::EdgeType,
//         Ix: petgraph::graph::IndexType + PartialEq,
//     {
//         let g_ns = g.node_references().map(|n| n.1).collect::<BTreeSet<_>>();
//         let g_es = g
//             .edge_indices()
//             .map(|e| {
//                 let endpoints = g.edge_endpoints(e).unwrap();
//                 (endpoints.0, endpoints.1, g.edge_weight(e).unwrap())
//             })
//             .collect::<BTreeSet<_>>();
//
//         (g_ns, g_es)
//     }
//
//     #[test]
//     fn parse_string() {
//         let check = |str_raw: &str, str_expected: &str| {
//             assert_eq!(
//                 graph_repr(&parse::<Value>(str_raw).unwrap()),
//                 graph_repr(&StableDiGraph::from_elements([Element::Node {
//                     weight: Entity::String(str_expected.to_string())
//                 }])),
//             );
//         };
//
//         check("test\n", "test");
//         check("$escaped\n", "$escaped");
//         check("0.0.1\n", "0.0.1");
//         check("5    GiB\n", "5    GiB");
//         check("'0.1'\n", "0.1");
//         check("''\n", "");
//         check("\"\"\n", "");
//         check("\"😏\"\n", "😏");
//     }
//
//     #[test]
//     fn parse_integer() {
//         let check = |int_raw: &str, int_expected: i64| {
//             assert_eq!(
//                 graph_repr(&parse::<Value>(int_raw).unwrap()),
//                 graph_repr(&StableDiGraph::from_elements([Element::Node {
//                     weight: Entity::Integer(int_expected)
//                 }])),
//             );
//         };
//
//         check("42\n", 42);
//         check("-42\n", -42);
//         check("0\n", 0);
//         check("-0\n", 0);
//         check("0x10\n", 16);
//         check("0o10\n", 8);
//     }
//
//     #[test]
//     fn parse_float() {
//         let check = |float_raw: &str, float_expected: &str| {
//             assert_eq!(
//                 graph_repr(&parse::<Value>(float_raw).unwrap()),
//                 graph_repr(&StableDiGraph::from_elements([Element::Node {
//                     weight: Entity::Float(float_expected.to_string())
//                 }])),
//             );
//         };
//
//         check("0.1\n", "0.1");
//         check("2e4\n", "2e4");
//         check(".inf\n", "inf");
//         check(".nan\n", "NaN");
//     }
//
//     #[test]
//     fn parse_bool() {
//         let check = |bool_raw: &str, bool_expected: bool| {
//             assert_eq!(
//                 graph_repr(&parse::<Value>(bool_raw).unwrap()),
//                 graph_repr(&StableDiGraph::from_elements([Element::Node {
//                     weight: Entity::Boolean(bool_expected)
//                 }])),
//             );
//         };
//
//         check("true\n", true);
//         check("True\n", true);
//         check("false\n", false);
//         check("False\n", false);
//     }
//
//     #[test]
//     fn parse_nil() {
//         assert_eq!(
//             graph_repr(&parse::<Value>("null\n").unwrap()),
//             graph_repr(&StableDiGraph::from_elements([Element::Node {
//                 weight: Entity::Nil
//             }])),
//         );
//     }
//
//     #[test]
//     fn parse_list() {
//         const LONG_LIST: &str = "
// - a
// - 1.2
// - 42\n";
//
//         const SHORT_LIST: &str = "[a, 1.2, 42]";
//
//         let check: StableDiGraph<_, _> = StableDiGraph::from_elements([
//             Element::Node {
//                 weight: Entity::List,
//             },
//             Element::Node {
//                 weight: Entity::String("a".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::Float("1.2".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::Integer(42),
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 1,
//                 weight: Relation::ListValue(0),
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 2,
//                 weight: Relation::ListValue(1),
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 3,
//                 weight: Relation::ListValue(2),
//             },
//         ]);
//
//         let long = parse::<Value>(LONG_LIST).unwrap();
//         let short = parse::<Value>(SHORT_LIST).unwrap();
//
//         let check_repr = graph_repr(&check);
//         assert_eq!(graph_repr(&long), check_repr);
//         assert_eq!(graph_repr(&short), check_repr);
//     }
//
//     #[test]
//     fn parse_map() {
//         const LONG_MAP: &str = "
// str_key: a
// 42: 1.2
// {
//     map: key
// }: 42\n";
//
//         const SHORT_MAP: &str = "{ str_key: a, 42: 1.2, { map: key }: 42 }";
//
//         let check: StableDiGraph<_, _> = StableDiGraph::from_elements([
//             Element::Node {
//                 weight: Entity::Map,
//             },
//             Element::Node {
//                 weight: Entity::String("str_key".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::String("a".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::Integer(42),
//             },
//             Element::Node {
//                 weight: Entity::Float("1.2".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::Map,
//             },
//             Element::Node {
//                 weight: Entity::String("map".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::String("key".to_string()),
//             },
//             Element::Node {
//                 weight: Entity::Integer(42),
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 1,
//                 weight: Relation::MapKey,
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 3,
//                 weight: Relation::MapKey,
//             },
//             Element::Edge {
//                 source: 1,
//                 target: 2,
//                 weight: Relation::MapValue,
//             },
//             Element::Edge {
//                 source: 3,
//                 target: 4,
//                 weight: Relation::MapValue,
//             },
//             Element::Edge {
//                 source: 5,
//                 target: 6,
//                 weight: Relation::MapKey,
//             },
//             Element::Edge {
//                 source: 6,
//                 target: 7,
//                 weight: Relation::MapValue,
//             },
//             Element::Edge {
//                 source: 0,
//                 target: 5,
//                 weight: Relation::MapKey,
//             },
//             Element::Edge {
//                 source: 5,
//                 target: 8,
//                 weight: Relation::MapValue,
//             },
//         ]);
//
//         let long = parse::<Value>(LONG_MAP).unwrap();
//         let short = parse::<Value>(SHORT_MAP).unwrap();
//
//         let check_repr = graph_repr(&check);
//         assert_eq!(graph_repr(&long), check_repr);
//         assert_eq!(graph_repr(&short), check_repr);
//     }
// }
