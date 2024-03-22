pub mod grammar;
pub mod models;
pub mod semantic;

use grammar::parser::ToscaGrammar;
pub use models::*;
use semantic::{Importer, TypeResolver};
use toto_parser::EntityParser;

pub struct ToscaParser;

impl ToscaParser {
    pub fn parse<E, R>(doc_root: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ToscaGrammar::parse(doc_root, ast)
            .into_iter()
            .map(|file_handle| Importer::import_files(file_handle, ast))
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|file_handle| Importer::import_types(file_handle, ast));
        // .map(|file_handle| {
        //     for _ in 1..10 {
        //         TypeResolver::parse(file_handle, ast);
        //     }
        //     file_handle
        // })
    }
}
