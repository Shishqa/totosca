#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

#[derive(Debug)]
pub struct ParseErrorLoc;
