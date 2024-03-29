use std::marker::PhantomData;

use crate::{ParseCompatibleEntity, ParseCompatibleRelation};

pub struct Field<C, V>(PhantomData<(C, V)>);

impl<C, V, E, R> toto_ast::RelationParser<E, R> for Field<C, V>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
    C: toto_ast::Linker<(), R>,
    V: toto_ast::EntityParser<E, R>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(n_handle) = V::parse(n, ast) {
            ast.add_edge(root, n_handle, C::L(()));
        }
    }
}
