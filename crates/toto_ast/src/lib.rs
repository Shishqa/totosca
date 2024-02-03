use std::fmt::Debug;

pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub trait Entity: Debug {}
pub trait Relation: Debug {}

pub type AST = petgraph::Graph<Box<dyn Entity>, Box<dyn Relation>, petgraph::Directed, u32>;

pub trait ToAST {
    fn to_ast(self, ast: &mut AST) -> GraphHandle;
}
