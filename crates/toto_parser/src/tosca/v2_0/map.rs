use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::{
    parse::{Context, Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct Map<PK: Parse, PV: Parse> {
    _k: PhantomData<PK>,
    _v: PhantomData<PV>,
}

impl<PK, PV> Parse for Map<PK, PV>
where
    PK: Parse,
    PV: Parse,
{
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::Map);

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
                    let key = PK::parse::<V>(ctx, &entry.0);
                    let value = PV::parse::<V>(ctx, &entry.1);

                    ctx.graph.add_edge(root, key, Relation::MapKey);
                    ctx.graph.add_edge(key, value, Relation::MapValue);
                });
            });

        root
    }
}