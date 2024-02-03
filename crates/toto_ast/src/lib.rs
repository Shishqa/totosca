pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub type AST<E, R> = petgraph::Graph<E, R, petgraph::Directed, u32>;

pub trait ToAST<E, R> {
    fn to_ast(self, ast: &mut AST<E, R>) -> GraphHandle;
}

pub trait TransformAST<E, R> {
    fn transform_ast(n: GraphHandle, ast: &mut AST<E, R>) -> GraphHandle;
}
