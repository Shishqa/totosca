use super::{NodeTemplates, ParameterDefinitions};
use crate::parse::{get_field, Error, Parse, ParseError};

#[derive(Debug)]
pub struct ServiceTemplateDefinition {
    pub description: Option<String>,
    pub inputs: ParameterDefinitions,
    pub node_templates: NodeTemplates,
    pub outputs: ParameterDefinitions,
}

impl Parse for ServiceTemplateDefinition {
    fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let mut description: Option<String> = None;
        let mut inputs: Option<ParameterDefinitions> = None;
        let mut outputs: Option<ParameterDefinitions> = None;
        let mut node_templates: Option<NodeTemplates> = None;

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "description" => get_field(entry.1, &mut description, &mut errors),
                    "inputs" => get_field(entry.1, &mut inputs, &mut errors),
                    "outputs" => get_field(entry.1, &mut outputs, &mut errors),
                    "node_templates" => get_field(entry.1, &mut node_templates, &mut errors),
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

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self {
                description,
                inputs: inputs.unwrap_or(ParameterDefinitions::new()),
                outputs: outputs.unwrap_or(ParameterDefinitions::new()),
                node_templates: node_templates.unwrap_or(NodeTemplates::new()),
            })
        }
    }
}
