use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct StringValue;
impl<E, R> toto_parser::EntityParser<E, R> for StringValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        toto_yaml::as_string(n, ast).map(|_| n).or_else(|| {
            add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
            None
        })
    }
}

pub struct BoolValue;
impl<E, R> toto_parser::EntityParser<E, R> for BoolValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        toto_yaml::as_bool(n, ast).map(|_| n).or_else(|| {
            add_with_loc(toto_parser::ParseError::UnexpectedType("bool"), n, ast);
            None
        })
    }
}

pub struct AnyValue;
impl<E, R> toto_parser::EntityParser<E, R> for AnyValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        _: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        Some(n)
    }
}
