use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct String;
impl<E, R> toto_ast::EntityParser<E, R> for String
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
where {
        toto_yaml::as_string(n, ast)
            .or_else(|| {
                add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
                None
            })
            .map(|_| n)
    }
}

pub struct Metadata;
impl<R> toto_ast::Linker<std::string::String, R> for Metadata
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: std::string::String) -> R = |s| crate::Relation::Metadata(s).into();
}

pub struct Description;
impl<R> toto_ast::Linker<(), R> for Description
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::Description.into();
}

pub struct Profile;
impl<R> toto_ast::Linker<(), R> for Profile
where
    R: ToscaCompatibleRelation,
{
    const L: fn(v: ()) -> R = |_| crate::Relation::Profile.into();
}
