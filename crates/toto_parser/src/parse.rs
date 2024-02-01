use std::path::{Path, PathBuf};

use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use toto_tosca::{Entity, Relation};

use crate::grammar::Grammar;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

#[derive(Debug)]
pub struct Error {
    pub pos: Option<u64>,
    pub error: ParseError,
}

pub type GraphHandle = NodeIndex;

pub struct Context {
    pub(crate) graph: StableDiGraph<Entity, Relation>,
    pub(crate) errors: Vec<Error>,
}

pub fn parse<G: Grammar, P: AsRef<Path>>(
    path: P,
) -> Result<StableDiGraph<Entity, Relation>, Vec<Error>> {
    let mut ctx = Context {
        graph: StableDiGraph::new(),
        errors: vec![],
    };

    G::parse(path, &mut ctx);

    if ctx.errors.is_empty() {
        Ok(ctx.graph)
    } else {
        Err(ctx.errors)
    }
}
