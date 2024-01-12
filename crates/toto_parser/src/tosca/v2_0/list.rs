use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{Context, Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
};

pub struct List<P: Parse> {
    _e: PhantomData<P>,
}

pub fn parse_list<P: Parse, V: ToscaDefinitionsVersion>(
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
            for (idx, n) in seq.iter().enumerate() {
                let elem = P::parse::<V>(ctx, &n);
                ctx.graph
                    .add_edge(root, elem, Relation::ListValue(idx as u64));
            }
        });
}

impl<P> Parse for List<P>
where
    P: Parse,
{
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::List);
        parse_list::<P, V>(ctx, root, n);
        root
    }
}
