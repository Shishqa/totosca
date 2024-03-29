pub mod collection;
pub mod error;
pub mod field;
pub mod list;
pub mod parse;
pub mod schema;

pub use collection::*;
pub use error::*;
pub use field::*;
pub use list::*;
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

pub trait Linker<V, R> {
    const L: fn(v: V) -> R;
}
