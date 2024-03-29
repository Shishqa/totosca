use petgraph::{Directed, Graph};

pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub type AST<E, R> = Graph<E, R, Directed, u32>;
