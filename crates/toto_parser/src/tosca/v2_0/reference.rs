use toto_tosca::Entity;

use crate::{
    parse::{Context, GraphHandle},
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

pub struct Reference;

impl Parse for Reference {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let s = String::from_yaml(n)
            .map_err(|err| ctx.errors.push(err))
            .unwrap_or_default();
        ctx.graph.add_node(Entity::Ref(s))
    }
}
