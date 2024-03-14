use crate::parse::{add_error, EntityParser, ParseError, ParseLoc};

use super::{
    v1_3,
    v2_0::{self, Tosca2_0},
    ToscaDefinitionsVersion,
};

pub trait ToscaCompatibleEntity:
    toto_yaml::AsYamlEntity
    + From<toto_yaml::FileEntity>
    + From<toto_yaml::Entity>
    + From<ParseError>
    + From<toto_tosca::Entity>
    + 'static
{
}

impl<T> ToscaCompatibleEntity for T where
    T: toto_yaml::AsYamlEntity
        + From<toto_yaml::FileEntity>
        + From<toto_yaml::Entity>
        + From<ParseError>
        + From<toto_tosca::Entity>
        + 'static
{
}

pub trait ToscaCompatibleRelation:
    toto_yaml::AsYamlRelation
    + From<toto_yaml::FileRelation>
    + From<toto_yaml::Relation>
    + From<ParseLoc>
    + From<toto_tosca::Relation>
    + 'static
{
}

impl<T> ToscaCompatibleRelation for T where
    T: toto_yaml::AsYamlRelation
        + From<toto_yaml::FileRelation>
        + From<toto_yaml::Relation>
        + From<ParseLoc>
        + From<toto_tosca::Relation>
        + 'static
{
}

pub struct ToscaParser {}

impl ToscaParser {
    pub fn parse<E, R>(doc: &str, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let doc_root = toto_yaml::YamlParser::parse(doc, ast);
        toto_yaml::as_map(doc_root, ast)
            .or_else(|| {
                add_error(doc_root, ast, ParseError::UnexpectedType("map"));
                None
            })
            .and_then(|items| {
                items.into_iter().find(|(k, v)| {
                    toto_yaml::as_string(*k, ast)
                        .and_then(|key| {
                            if key == "tosca_definitions_version" {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .is_some()
                })
            })
            .or_else(|| {
                add_error(
                    doc_root,
                    ast,
                    ParseError::MissingField("tosca_definitions_version"),
                );
                None
            })
            .and_then(|(_, v)| toto_yaml::as_string(v, ast))
            .or_else(|| {
                add_error(doc_root, ast, ParseError::UnexpectedType("string"));
                None
            })
            .and_then(|version| match version.as_str() {
                "tosca_2_0" => <v2_0::Tosca2_0 as ToscaDefinitionsVersion>::FileDefinition::parse(
                    doc_root, ast,
                ),
                "tosca_simple_yaml_1_3" => {
                    <v1_3::Tosca1_3 as ToscaDefinitionsVersion>::FileDefinition::parse(
                        doc_root, ast,
                    )
                }
                _ => None,
            });
        doc_root
    }
}
