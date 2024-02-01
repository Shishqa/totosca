use std::{fs, path::Path};

use petgraph::data::Build;
use toto_tosca::Entity;
use url::Url;
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
    fn parse<P: AsRef<Path>>(path: P, ctx: &mut Context) {
        let doc = fs::read_to_string(path.as_ref()).map_err(|err| {
            ctx.errors.push(Error {
                pos: None,
                error: ParseError::Custom(format!(
                    "{}: {}",
                    path.as_ref().display(),
                    err.to_string()
                )),
            })
        });
        if let Err(_) = doc {
            return;
        }
        let doc = doc.unwrap();

        let mut n = yaml_peg::parse::<yaml_peg::repr::RcRepr>(&doc)
            .map_err(|err| {
                ctx.errors.push(Error {
                    pos: None,
                    error: ParseError::Custom(format!("cannot parse yaml: {:?}", err)),
                })
            })
            .unwrap_or_default();

        if !ctx.errors.is_empty() {
            return;
        }

        let n = n.remove(0);

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

    fn resolve(_ctx: &mut Context) {}
}
