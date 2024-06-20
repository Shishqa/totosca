use toto_parser::add_with_loc;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct ToscaGrammar;

impl ToscaGrammar {
    pub fn get_tosca_version<E, R>(
        doc_root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<(toto_ast::GraphHandle, String)>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
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
                Some(version) => Some((v, version.0.clone())),
                _ => {
                    add_with_loc(
                        toto_parser::ParseError::UnexpectedType("string"),
                        doc_root,
                        ast,
                    );
                    None
                }
            })
    }
}
