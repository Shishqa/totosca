use petgraph::{Directed, Graph};

pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub type AST<E, R> = Graph<E, R, Directed, u32>;

pub trait EntityParser<E, R> {
    fn parse(n: GraphHandle, ast: &mut AST<E, R>) -> Option<GraphHandle>;
}

pub trait RelationParser<E, R> {
    fn parse(root: GraphHandle, n: GraphHandle, ast: &mut AST<E, R>);
}

pub trait Linker<V, R> {
    const L: fn(v: V) -> R;
}
