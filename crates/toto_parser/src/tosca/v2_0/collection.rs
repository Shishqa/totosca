use toto_tosca::Relation;

use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
    yaml::FromYaml,
};

pub fn parse_collection<P: Parse, V: ToscaDefinitionsVersion>(
    ctx: &mut toto_ast::AST,
    root: toto_ast::GraphHandle,
    n: &yaml_peg::NodeRc,
) {
    let _ = n
        .as_map()
        .map_err(|pos| {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(pos),
                error: ParseErrorKind::UnexpectedType("map"),
            }))
        })
        .map(|mut m| {
            m.drain().for_each(|entry| {
                let _ = String::from_yaml(&entry.0)
                    .map_err(|err| ctx.errors.push(Box::new(err)))
                    .map(|key| {
                        let value = P::parse::<V>(ctx, &entry.1);
                        ctx.graph.add_edge(root, value, Relation::Subdef(key));
                    });
            });
        });
}
