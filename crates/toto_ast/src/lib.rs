use petgraph::{stable_graph::StableGraph, Directed};

pub type GraphHandle = petgraph::graph::NodeIndex<u32>;
pub type EdgeHandle = petgraph::graph::EdgeIndex<u32>;

pub type AST<E, R> = StableGraph<E, R, Directed, u32>;
