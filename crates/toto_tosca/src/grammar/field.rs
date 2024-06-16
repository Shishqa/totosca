use std::marker::PhantomData;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Field<C, V>(PhantomData<(C, V)>);

impl<C, V, E, R> toto_parser::RelationParser<E, R> for Field<C, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    C: Default,
    V: toto_parser::EntityParser<E, R>,
    crate::Relation: From<C>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        if let Some(n_handle) = V::parse(n, ast) {
            ast.add_edge(root, n_handle, crate::Relation::from(C::default()).into());
            ast.add_edge(
                n_handle,
                root,
                crate::Relation::Root(crate::RootRelation).into(),
            );
        }
    }
}

impl<C, V, E, R, Value> toto_parser::ValueRelationParser<E, R, Value> for Field<C, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    C: From<Value>,
    V: toto_parser::EntityParser<E, R>,
    crate::Relation: From<C>,
{
    fn parse(
        v: Value,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) {
        if let Some(n_handle) = V::parse(n, ast) {
            ast.add_edge(root, n_handle, crate::Relation::from(C::from(v)).into());
            ast.add_edge(
                n_handle,
                root,
                crate::Relation::Root(crate::RootRelation).into(),
            );
        }
    }
}
