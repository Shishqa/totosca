use crate::parse::{Error, Parse, ParseError};
use std::collections::BTreeMap;

pub type Map<K, V> = BTreeMap<K, V>;

impl<K, V> Parse for Map<K, V>
where
    K: Parse + std::cmp::Ord,
    V: Parse,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let m = n
            .as_map()
            .map_err(|pos| {
                errors.push(Error {
                    pos: Some(pos),
                    error: ParseError::UnexpectedType("map"),
                })
            })
            .map(|mut m| {
                let mut s = Self::new();
                m.drain().for_each(|e| {
                    let k = K::from_yaml(&e.0).map_err(|e| errors.extend(e)).ok();

                    let v = V::from_yaml(&e.1).map_err(|e| errors.extend(e)).ok();

                    if k.is_some() && v.is_some() {
                        s.insert(k.unwrap(), v.unwrap());
                    }
                });
                s
            });

        if errors.is_empty() {
            Ok(m.unwrap())
        } else {
            Err(errors)
        }
    }
}
