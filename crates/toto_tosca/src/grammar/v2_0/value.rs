use petgraph::data::DataMap;
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

pub struct NullableStringValue;
impl<E, R> toto_parser::EntityParser<E, R> for NullableStringValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        match ast.node_weight(n).unwrap().as_yaml() {
            Some(toto_yaml::Entity::Str(_) | toto_yaml::Entity::Null(_)) => Some(n),
            _ => {
                add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
                None
            }
        }
    }
}

pub struct IntValue;
impl<E, R> toto_parser::EntityParser<E, R> for IntValue
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        toto_yaml::as_int(n, ast).map(|_| n).or_else(|| {
            add_with_loc(toto_parser::ParseError::UnexpectedType("integer"), n, ast);
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
