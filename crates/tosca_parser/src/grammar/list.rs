use crate::parse::{Error, Parse, ParseError};
use std::vec::Vec;

pub type List<T> = Vec<T>;

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let l = n
            .as_seq()
            .map_err(|pos| {
                errors.push(Error {
                    pos: Some(pos),
                    error: ParseError::UnexpectedType("list"),
                })
            })
            .map(|seq| {
                let mut values = vec![];
                for n in seq {
                    match T::from_yaml(&n) {
                        Ok(v) => values.push(v),
                        Err(e) => errors.extend(e),
                    }
                }
                values
            });

        if errors.is_empty() {
            Ok(l.unwrap())
        } else {
            Err(errors)
        }
    }
}
