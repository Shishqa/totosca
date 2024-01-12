mod value;

pub use value::*;

use crate::parse::Error;

pub trait FromYaml
where
    Self: Sized,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Error>;
}
