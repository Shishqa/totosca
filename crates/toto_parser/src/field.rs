use std::marker::PhantomData;

use crate::{EntityParser, Linker, ParseCompatibleEntity, ParseCompatibleRelation, RelationParser};

pub struct Field<C, V>(PhantomData<(C, V)>);

impl<C, V, E, R> RelationParser<E, R> for Field<C, V>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
    C: Linker<(), R>,
    V: EntityParser<E, R>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        V::parse(n, ast).and_then(|n_handle| {
            ast.add_edge(root, n_handle, C::L(()));
            Some(n_handle)
        });
    }
}
