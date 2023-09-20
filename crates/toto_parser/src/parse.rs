use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use toto_tosca::{Entity, Relation};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(&'static str),
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

pub trait FromYaml
where
    Self: Sized,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error>;
}

pub trait Parse {
    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle;
}

pub fn parse<P: Parse>(doc: &str) -> Result<StableDiGraph<Entity, Relation>, Vec<Error>> {
    let result = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
        .unwrap()
        .remove(0);

    let mut ctx = Context {
        graph: StableDiGraph::new(),
        errors: vec![],
    };

    let root = P::parse(&mut ctx, &result);

    if ctx.errors.is_empty() {
        Ok(ctx.graph)
    } else {
        Err(ctx.errors)
    }
}
