use std::marker::PhantomData;

use toto_tosca::{Entity, Relation};

use crate::parse::{Context, Error, GraphHandle, Parse, ParseError};

#[derive(Debug)]
pub struct Map<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> Parse for Map<K, V>
where
    K: Parse,
    V: Parse,
{
    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
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
                    let key = K::parse(ctx, &entry.0);
                    let value = V::parse(ctx, &entry.1);

                    ctx.graph.add_edge(root, key, Relation::MapKey);
                    ctx.graph.add_edge(key, value, Relation::MapValue);
                });
            });

        root
    }
}
