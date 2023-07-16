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

pub trait Parse
where
    Self: Sized,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>>;
}

pub fn parse<P: Parse>(doc: &str) -> Result<P, Vec<Error>> {
    let result = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
        .unwrap()
        .remove(0);
    P::from_yaml(&result)
}

pub fn get_field<T: Parse>(n: &yaml_peg::NodeRc, dest: &mut Option<T>, err: &mut Vec<Error>) {
    *dest = T::from_yaml(n).map_err(|e| err.extend(e)).ok();
}

// TODO: move somewhere?
impl<T: Parse> Parse for Box<T> {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        T::from_yaml(n).map(|value| Box::new(value))
    }
}
