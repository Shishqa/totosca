use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

pub struct List<P: Parse> {
    _e: PhantomData<P>,
}

pub fn parse_list<P: Parse, V: ToscaDefinitionsVersion>(
    ctx: &mut toto_ast::AST,
    root: toto_ast::GraphHandle,
    n: &yaml_peg::NodeRc,
) {
    n.as_seq()
        .map_err(|pos| {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(pos),
                error: ParseErrorKind::UnexpectedType("list"),
            }))
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
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
        let root = ctx.graph.add_node(Entity::List);
        parse_list::<P, V>(ctx, root, n);
        root
    }
}
