use super::List;
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct ImportDefinition {
    pub url: Option<String>,
    pub profile: Option<String>,
    pub repository: Option<String>,
    pub namespace: Option<String>,
}

pub type ImportDefinitions = List<ImportDefinition>;

impl Parse for ImportDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut url: Option<String> = None;
        let mut profile: Option<String> = None;
        let mut repository: Option<String> = None;
        let mut namespace: Option<String> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "url" => get_field(entry.1, &mut url, &mut errors),
                    "profile" => get_field(entry.1, &mut profile, &mut errors),
                    "repository" => get_field(entry.1, &mut repository, &mut errors),
                    "namespace" => get_field(entry.1, &mut namespace, &mut errors),
                    f => errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else if let Ok(s) = n.as_str() {
            url = Some(s.to_string());
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map or string"),
            });
            return Err(errors);
        }

        if url.is_none() && profile.is_none() {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("url or profile"),
            });
        } else if url.is_some() && profile.is_some() {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::Custom("url and profile fields are mutually exclusive"),
            });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                url,
                profile,
                repository,
                namespace,
            })
        }
    }
}
