use std::fmt::{Debug, Display};

use petgraph::{data::DataMap, visit::EdgeRef, Direction::Outgoing};
use toto_yaml::{AsFileEntity, AsFileRelation, FileRelation};

#[derive(Debug, Clone)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::Custom(err) => f.write_str(&err),
            &Self::MissingField(f_name) => f.write_fmt(format_args!("missing field: {}", f_name)),
            &Self::UnknownField(_) => f.write_str("unsupported field"),
            &Self::UnexpectedType(t_name) => f.write_fmt(format_args!("expected {}", t_name)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseLoc;

pub trait AsParseError {
    fn as_parse(&self) -> Option<&ParseError>;
}

pub trait AsParseLoc {
    fn as_parse_loc(&self) -> Option<&ParseLoc>;
}

pub trait ParseCompatibleEntity:
    toto_yaml::AsYamlEntity + AsFileEntity + AsParseError + From<ParseError> + Debug + 'static
{
}
pub trait ParseCompatibleRelation:
    toto_yaml::AsYamlRelation
    + AsFileRelation
    + AsParseLoc
    + From<ParseLoc>
    + From<FileRelation>
    + Debug
    + 'static
{
}

impl<T> ParseCompatibleEntity for T where
    T: toto_yaml::AsYamlEntity + AsFileEntity + AsParseError + From<ParseError> + Debug + 'static
{
}
impl<T> ParseCompatibleRelation for T where
    T: toto_yaml::AsYamlRelation
        + AsFileRelation
        + AsParseLoc
        + From<ParseLoc>
        + From<FileRelation>
        + Debug
        + 'static
{
}

pub fn add_with_loc<E, R>(
    e: impl Into<E>,
    loc: toto_ast::GraphHandle,
    ast: &mut toto_ast::AST<E, R>,
) -> toto_ast::GraphHandle
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    let n = ast.add_node(e.into());

    let (pos, file) = if ast.node_weight(loc).unwrap().as_file().is_some() {
        (0, loc)
    } else {
        ast.edges_directed(loc, Outgoing)
            .find_map(|e| e.weight().as_file().map(|pos| (pos.0, e.target())))
            .unwrap()
    };

    ast.add_edge(n, loc, ParseLoc.into());
    ast.add_edge(n, file, toto_yaml::FileRelation(pos).into());
    n
}
