use std::marker::PhantomData;

use toto_parser::add_with_loc;

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};

#[derive(Debug)]
pub struct ImportDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_ast::EntityParser<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let import = add_with_loc(crate::Entity::Definition, n, ast);
        toto_yaml::as_string(n, ast)
            .and_then(|_| {
                ast.add_edge(import, n, crate::Relation::Url.into());
                Some(import)
            })
            .or_else(|| {
                add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
                None
            });
        Some(import)
    }
}
