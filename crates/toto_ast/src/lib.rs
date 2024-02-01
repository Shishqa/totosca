use std::collections::HashMap;

use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use toto_tosca::{Entity, Relation};

pub type GraphHandle = NodeIndex;

pub trait Error {
    fn loc(&self) -> u64;
    fn what(&self) -> String;
}

pub struct AST {
    pub files: HashMap<String, GraphHandle>,
    pub graph: StableDiGraph<Entity, Relation>,
    pub errors: Vec<Box<dyn Error>>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            graph: StableDiGraph::new(),
            errors: vec![],
        }
    }
}
