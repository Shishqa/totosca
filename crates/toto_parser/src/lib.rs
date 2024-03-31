pub mod error;
pub mod parse;
pub mod schema;

pub use error::*;
pub use parse::*;
pub use schema::*;

pub trait EntityParser<E, R> {
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>;
}

pub trait RelationParser<E, R> {
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>);
}
