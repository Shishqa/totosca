pub trait Grammar {
    fn parse(doc: &str, ctx: &mut toto_ast::AST) -> Option<toto_ast::GraphHandle>;
}
