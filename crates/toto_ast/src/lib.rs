use std::collections::HashMap;

use toto_tosca::{Entity, Relation};

pub type GraphHandle = petgraph::graph::NodeIndex<u32>;

pub trait Error {
    fn loc(&self) -> u64;
    fn what(&self) -> String;
}

pub struct AST {
    pub files: HashMap<String, GraphHandle>,
    pub graph: petgraph::Graph<Entity, Relation, petgraph::Directed, u32>,
    pub errors: Vec<Box<dyn Error>>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            graph: petgraph::Graph::new(),
            errors: vec![],
        }
    }
}
