use petgraph::{Directed, Graph};

pub type AST<E, R> = Graph<E, R, Directed, u32>;
pub type GraphHandle = petgraph::graph::NodeIndex<u32>;
