use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::parse::{Context, Error, GraphHandle, Parse, ParseError};

pub struct List<T> {
    _e: PhantomData<T>,
}

pub fn parse_list<T: Parse>(ctx: &mut Context, root: GraphHandle, n: &yaml_peg::NodeRc) {
    n.as_seq()
        .map_err(|pos| {
            ctx.errors.push(Error {
                pos: Some(pos),
                error: ParseError::UnexpectedType("list"),
            })
        })
        .map(|seq| {
            for (idx, n) in seq.iter().enumerate() {
                let elem = T::parse(ctx, &n);
                ctx.graph
                    .add_edge(root, elem, Relation::ListValue(idx as u64));
            }
        });
}

impl<T> Parse for List<T>
where
    T: Parse,
{
    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::List);
        parse_list::<T>(ctx, root, n);
        root
    }
}
