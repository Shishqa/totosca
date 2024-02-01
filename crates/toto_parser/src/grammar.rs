use std::path::Path;

pub trait Grammar {
    fn parse<P: AsRef<Path>>(path: P, ctx: &mut toto_ast::AST);
}
