use super::{DataTypes, ImportDefinitions, Map, NodeTypes, ServiceTemplateDefinition};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct ToscaFileDefinition {
    pub tosca_definitions_version: ToscaVersion,
    pub profile: Option<String>,
    pub metadata: Map<String, String>,
    pub description: Option<String>,
    pub imports: ImportDefinitions,
    pub data_types: DataTypes,
    pub node_types: NodeTypes,
    pub service_template: Option<ServiceTemplateDefinition>,
}

impl Parse for ToscaFileDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut tosca_version: Option<ToscaVersion> = None;
        let mut profile: Option<String> = None;
        let mut metadata: Option<Map<String, String>> = None;
        let mut description: Option<String> = None;
        let mut imports: Option<ImportDefinitions> = None;
        let mut data_types: Option<DataTypes> = None;
        let mut node_types: Option<NodeTypes> = None;
        let mut service_template: Option<ServiceTemplateDefinition> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "tosca_definitions_version" => {
                        get_field(entry.1, &mut tosca_version, &mut errors)
                    }
                    "profile" => get_field(entry.1, &mut profile, &mut errors),
                    "metadata" => get_field(entry.1, &mut metadata, &mut errors),
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "imports" => get_field(entry.1, &mut imports, &mut errors),
                    "data_types" => get_field(entry.1, &mut data_types, &mut errors),
                    "node_types" => get_field(entry.1, &mut node_types, &mut errors),
                    "service_template" => get_field(entry.1, &mut service_template, &mut errors),
                    f => errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return Err(errors);
        }

        if tosca_version.is_none() {
            errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::MissingField("tosca_definitions_version"),
            });
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                tosca_definitions_version: tosca_version.unwrap(),
                profile,
                description,
                metadata: metadata.unwrap_or(Map::new()),
                imports: imports.unwrap_or(ImportDefinitions::new()),
                data_types: data_types.unwrap_or(DataTypes::new()),
                node_types: node_types.unwrap_or(NodeTypes::new()),
                service_template,
            })
        }
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
            _ => Err(vec![Error {
                pos: Some(n.pos()),
                error: ParseError::Custom("unknown TOSCA version"),
            }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn it_works() {
        const DOC: &str = include_str!("../tests/file.yaml");

        let _ = dbg!(parse::<ToscaFileDefinition>(DOC));
    }
}
