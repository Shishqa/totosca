use toto_yaml::{AsFileEntity, AsFileRelation};

#[derive(Debug, Clone)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
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
    toto_yaml::AsYamlEntity + AsFileEntity + AsParseError + From<ParseError> + 'static
{
}
pub trait ParseCompatibleRelation:
    toto_yaml::AsYamlRelation + AsFileRelation + AsParseLoc + From<ParseLoc> + 'static
{
}

impl<T> ParseCompatibleEntity for T where
    T: toto_yaml::AsYamlEntity + AsFileEntity + AsParseError + From<ParseError> + 'static
{
}
impl<T> ParseCompatibleRelation for T where
    T: toto_yaml::AsYamlRelation + AsFileRelation + AsParseLoc + From<ParseLoc> + 'static
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
    ast.add_edge(n, loc, ParseLoc.into());
    n
}
