use yaml_peg::node;

use crate::{
    grammar::Grammar,
    parse::{Context, Error, GraphHandle, ParseError},
};

use self::{v1_3::Tosca1_3, v2_0::Tosca2_0};

pub mod v1_3;
pub mod v2_0;

pub trait Parse {
    fn parse<V: ToscaDefinitionsVersion>(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle;
}

pub trait ToscaDefinitionsVersion {
    type AttributeDefinition: Parse;
    type AttributeAssignment: Parse;
    type PropertyDefinition: Parse;
    type PropertyAssignment: Parse;
    type ParameterDefinition: Parse;
    type DataTypeDefinition: Parse;
    type NodeTypeDefinition: Parse;
    type NodeTemplateDefinition: Parse;
    type RequirementDefinition: Parse;
    type RequirementAssignment: Parse;
    type SchemaDefinition: Parse;
    type ImportDefinition: Parse;
    type ServiceTemplateDefinition: Parse;
    type FileDefinition: Parse;
    type Value: Parse;

    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle;

    // TODO: here we can add url pointing to actual spec which can be useful in report printing
    // fn spec_url() -> &'static str;
}

pub struct ToscaGrammar;

impl Grammar for ToscaGrammar {
    fn parse(doc: &str, ctx: &mut Context) {
        let n = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        if let Ok(map) = n.as_map() {
            match map.get(&node!("tosca_definitions_version")) {
                Some(version) => match version.as_str() {
                    Ok(version_str) => match version_str {
                        "tosca_simple_yaml_1_3" => {
                            Tosca1_3::parse(ctx, &n);
                        }
                        "tosca_2_0" => {
                            Tosca2_0::parse(ctx, &n);
                        }
                        unknown => ctx.errors.push(Error {
                            pos: Some(n.pos()),
                            error: ParseError::Custom(format!(
                                "unknown tosca definitions version: {}",
                                unknown
                            )),
                        }),
                    },
                    Err(pos) => ctx.errors.push(Error {
                        pos: Some(pos),
                        error: ParseError::UnexpectedType("string"),
                    }),
                },
                None => ctx.errors.push(Error {
                    pos: Some(n.pos()),
                    error: ParseError::MissingField("tosca_definitions_version"),
                }),
            }
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            })
        }
    }
}
