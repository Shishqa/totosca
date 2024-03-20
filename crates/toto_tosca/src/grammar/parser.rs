use toto_parser::add_with_loc;

use crate::{semantic::Importer, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{v1_3::Tosca1_3, v2_0::Tosca2_0};

pub struct ToscaParser;

impl<E, R> toto_parser::EntityParser<E, R> for ToscaParser
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
                add_with_loc(
                    toto_parser::ParseError::MissingField("tosca_definitions_version"),
                    doc_root,
                    ast,
                );
                None
            })
            .and_then(|(_, v)| {
                toto_yaml::as_string(v, ast).or_else(|| {
                    add_with_loc(toto_parser::ParseError::UnexpectedType("string"), v, ast);
                    None
                })
            })
            .and_then(|version| match version.as_str() {
                "tosca_2_0" => Tosca2_0::parse(doc_root, ast),
                "tosca_simple_yaml_1_3" => Tosca1_3::parse(doc_root, ast),
                _ => None,
            })
            .and_then(|file_handle| Importer::parse(file_handle, ast))
    }
}
