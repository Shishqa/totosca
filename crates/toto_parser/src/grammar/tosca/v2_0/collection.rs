use toto_tosca::Relation;

use crate::parse::{Context, Error, FromYaml, GraphHandle, Parse, ParseError};

pub fn parse_collection<V: Parse>(ctx: &mut Context, root: GraphHandle, n: &yaml_peg::NodeRc) {
    let _ = n
        .as_map()
        .map_err(|pos| {
            ctx.errors.push(Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("map"),
            })
        })
        .map(|mut m| {
            m.drain().for_each(|entry| {
                String::from_yaml(&entry.0)
                    .map_err(|err| ctx.errors.push(err))
                    .map(|key| {
                        let value = V::parse(ctx, &entry.1);
                        ctx.graph.add_edge(root, value, Relation::Subdef(key));
                    });
            });
        });
}
