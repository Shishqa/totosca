use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{hierarchy::Hierarchy, v1_3::Tosca1_3, v2_0::Tosca2_0};

pub struct ToscaGrammar;

impl<E, R> toto_parser::EntityParser<E, R> for ToscaGrammar
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        doc_root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        toto_yaml::as_map(doc_root, ast)
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map"),
                    doc_root,
                    ast,
                );
                None
            })
            .and_then(|items| {
                items.into_iter().find(|(k, v)| {
                    toto_yaml::as_string(*k, ast)
                        .and_then(|key| {
                            if key.0 == "tosca_definitions_version" {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .is_some()
                })
            })
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::MissingField("tosca_definitions_version"),
                    doc_root,
                    ast,
                );
                None
            })
            .and_then(|(_, v)| match toto_yaml::as_string(v, ast) {
                Some(version) => match version.0.as_str() {
                    "tosca_2_0" => Tosca2_0::parse(doc_root, ast),
                    "tosca_simple_yaml_1_3" => Tosca1_3::parse(doc_root, ast),
                    _ => {
                        add_with_loc(
                            toto_parser::ParseError::Custom(format!(
                                "unknown version: {}",
                                version.0
                            )),
                            doc_root,
                            ast,
                        );
                        None
                    }
                },
                _ => {
                    add_with_loc(
                        toto_parser::ParseError::MissingField("tosca_definitions_version"),
                        doc_root,
                        ast,
                    );
                    None
                }
            })
            .map(|file_handle| Hierarchy::link(file_handle, ast))
    }
}
