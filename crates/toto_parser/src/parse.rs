#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

#[derive(Debug)]
pub struct ParseError {
    pub pos: Option<u64>,
    pub error: ParseErrorKind,
}

impl toto_ast::Error for ParseError {
    fn loc(&self) -> u64 {
        self.pos.unwrap_or_default()
    }

    fn what(&self) -> String {
        format!("{:?}", self.error)
    }
}
