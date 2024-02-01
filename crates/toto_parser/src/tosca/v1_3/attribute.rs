use toto_tosca::Relation;
use yaml_peg::node;

use crate::tosca::{Parse, ToscaDefinitionsVersion};

#[derive(Debug)]
pub struct AttributeAssignment;

impl Parse for AttributeAssignment {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        if let Ok(map) = n.as_map() {
            if let (Some(description), Some(value)) =
                (map.get(&node!("description")), map.get(&node!("value")))
            {
                let value = V::Value::parse::<V>(ctx, value);
                let t = String::parse::<V>(ctx, description);
                ctx.graph.add_edge(value, t, Relation::Description);
                return value;
            }
        }
        V::Value::parse::<V>(ctx, n)
    }
}
