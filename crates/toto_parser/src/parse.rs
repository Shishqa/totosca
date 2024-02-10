#[derive(Debug, Clone)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ParseLoc;
