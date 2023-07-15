use std::collections::BTreeMap as Map;
use yaml_peg;


#[derive(Debug)]
pub struct Error {
    pub pos: Option<u64>,
    pub error: String,
}

trait Parse
where Self: Sized
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>>;
}


impl<K, V> Parse for Map<K, V>
where
    K: Parse + std::cmp::Ord,
    V: Parse,
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let m = n
            .as_map()
            .map_err(|pos| errors.push(Error { pos: Some(pos), error: "expected map".to_string(), }))
            .map(|mut m| {
                let mut s = Self::new();
                m.drain().for_each(|e| {
                    let k = K::from_yaml(&e.0)
                        .map_err(|e| errors.extend(e))
                        .ok();

                    let v = V::from_yaml(&e.1)
                        .map_err(|e| errors.extend(e))
                        .ok();

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

impl Parse for String
{
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        n
            .as_str()
            .map_err(|pos| vec![Error { pos: Some(pos), error: "expected string".to_string(), }])
            .map(|s| s.to_string())
    }
}

#[derive(Debug)]
pub enum ToscaVersion {
    ToscaSimpleYaml1_3,
    Tosca2_0,
}

impl Parse for ToscaVersion {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let s = String::from_yaml(n)?;
        match s.as_str() {
            "tosca_simple_yaml_1_3" => Ok(ToscaVersion::ToscaSimpleYaml1_3),
            "tosca_2_0" => Ok(ToscaVersion::Tosca2_0),
            _ => Err(vec![Error { pos: Some(n.pos()), error: format!("unknown TOSCA version: {}", s) }]),
        }
    }
}

#[derive(Debug)]
pub struct NodeTypeDefinition {
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub metadata: Map<String, String>,
    pub version: Option<String>,
    pub extra: Option<String>,
}

impl Parse for NodeTypeDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut derived_from: Option<String> = None;
        let mut description: Option<String> = None;
        let mut version: Option<String> = None;
        let mut extra: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;

        if let Ok(map) = n.as_map() {
            map.iter().for_each(|entry| {
                match entry.0.as_str().unwrap() {
                    "derived_from" => {
                        derived_from = String::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok();
                    }
                    "description" => {
                        description = String::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok();
                    }
                    "version" => {
                        version = String::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok();
                    }
                    "extra" => {
                        extra = String::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok()
                    }
                    "metadata" => {
                        metadata = Map::<String, String>::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok()
                    }
                    _ => errors.push(Error { pos: Some(entry.0.pos()), error: "unknown field".to_string(), }),
                }
            });
        } else {
            errors.push(Error { pos: Some(n.pos()), error: "expected map".to_string(), });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                derived_from,
                version,
                description,
                extra,
                metadata: metadata.unwrap_or(Map::new()),
            })
        }
    }
}

#[derive(Debug)]
pub struct FileDefinition {
    pub tosca_definitions_version: ToscaVersion,

    pub metadata: Map<String, String>,

    pub description: Option<String>,

    pub node_types: Map<String, NodeTypeDefinition>,

    // TODO: add rest fields
}

impl Parse for FileDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut tosca_version: Option<ToscaVersion> = None;
        let mut description: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;
        let mut node_types: Option<Map<String, NodeTypeDefinition>> = None;

        if let Ok(map) = n.as_map() {
            map.iter().for_each(|entry| {
                match entry.0.as_str().unwrap() {
                    "tosca_definitions_version" => {
                        tosca_version = ToscaVersion::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok();
                    }
                    "description" => {
                        description = String::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok();
                    }
                    "metadata" => {
                        metadata = Map::<String, String>::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok()
                    }
                    "node_types" => {
                        node_types = Map::<String, NodeTypeDefinition>::from_yaml(entry.1)
                            .map_err(|e| errors.extend(e))
                            .ok()
                    }
                    _ => errors.push(Error { pos: Some(entry.0.pos()), error: "unknown field".to_string(), }),
                }
            });
        } else {
            errors.push(Error { pos: Some(n.pos()), error: "expected map".to_string(), });
        }

        if tosca_version.is_none() {
            errors.push(Error { pos: Some(n.pos()), error: "missing tosca_definitions_version".to_string(), });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                tosca_definitions_version: tosca_version.unwrap(),
                description,
                metadata: metadata.unwrap_or(Map::new()),
                node_types: node_types.unwrap_or(Map::new()),
            })
        }
    }
}

pub fn parse(doc: &str) -> Result<FileDefinition, Vec<Error>> {
    let result = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc).unwrap().remove(0);
    FileDefinition::from_yaml(&result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const DOC: &str = include_str!("tests/file.yaml");

        let res = dbg!(parse(DOC));
    }
}
