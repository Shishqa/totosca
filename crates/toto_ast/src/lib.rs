use petgraph::{Directed, Graph};

pub type AST<E, R> = Graph<E, R, Directed, u32>;
pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub trait Parse<E, R> {
    fn parse(self, ast: &mut AST<E, R>) -> GraphHandle;
}
