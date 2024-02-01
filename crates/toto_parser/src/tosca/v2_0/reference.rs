use toto_tosca::Entity;

use crate::{
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

pub struct Reference;

impl Parse for Reference {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let s = String::from_yaml(n)
            .map_err(|err| ctx.errors.push(Box::new(err)))
            .unwrap_or_default();
        ctx.graph.add_node(Entity::Ref(s))
    }
}
