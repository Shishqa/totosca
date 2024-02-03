use yaml_peg::node;

use crate::{
    grammar::Grammar,
    parse::{ParseError, ParseErrorKind},
};

use self::{v1_3::Tosca1_3, v2_0::Tosca2_0};

pub mod v1_3;
pub mod v2_0;

pub trait Parse {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle;
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

    fn parse(ctx: &mut toto_ast::AST, n: &yaml_peg::NodeRc) -> toto_ast::GraphHandle;

    // TODO: here we can add url pointing to actual spec which can be useful in report printing
    // fn spec_url() -> &'static str;
}

pub struct ToscaGrammar;

impl Grammar for ToscaGrammar {
    fn parse(doc: &str, ctx: &mut toto_ast::AST) -> Option<toto_ast::GraphHandle> {
        let mut n = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .map_err(|err| {
                ctx.errors.push(Box::new(ParseError {
                    pos: None,
                    error: ParseErrorKind::Custom(format!("cannot parse yaml: {:?}", err)),
                }))
            })
            .unwrap_or_default();

        if !ctx.errors.is_empty() {
            return None;
        }

        let n = n.remove(0);

        if let Ok(map) = n.as_map() {
            match map.get(&node!("tosca_definitions_version")) {
                Some(version) => match version.as_str() {
                    Ok(version_str) => match version_str {
                        "tosca_simple_yaml_1_3" => Some(Tosca1_3::parse(ctx, &n)),
                        "tosca_2_0" => Some(Tosca2_0::parse(ctx, &n)),
                        unknown => {
                            ctx.errors.push(Box::new(ParseError {
                                pos: Some(n.pos()),
                                error: ParseErrorKind::Custom(format!(
                                    "unknown tosca definitions version: {}",
                                    unknown
                                )),
                            }));
                            None
                        }
                    },
                    Err(pos) => {
                        ctx.errors.push(Box::new(ParseError {
                            pos: Some(pos),
                            error: ParseErrorKind::UnexpectedType("string"),
                        }));
                        None
                    }
                },
                None => {
                    ctx.errors.push(Box::new(ParseError {
                        pos: Some(n.pos()),
                        error: ParseErrorKind::MissingField("tosca_definitions_version"),
                    }));
                    None
                }
            }
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType("map"),
            }));
            None
        }
    }
}
