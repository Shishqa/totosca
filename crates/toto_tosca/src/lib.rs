pub mod grammar;
pub mod models;
pub mod semantic;

use grammar::parser::ToscaGrammar;
pub use models::*;
use semantic::{Hierarchy, Importer, Lookup};
use toto_parser::EntityParser;

pub struct ToscaParser;

impl ToscaParser {
    pub fn parse<E, R>(doc_root: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>)
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ToscaGrammar::parse(doc_root, ast)
            .map(|file_handle| Hierarchy::link(file_handle, ast))
            .into_iter()
            .flat_map(|file_handle| Importer::import_files(file_handle, ast))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|file_handle| {
                Importer::import_types(file_handle, ast);
            });

        Lookup::lookup(ast);
    }
}
