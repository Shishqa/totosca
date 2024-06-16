pub mod grammar;
pub mod models;
pub mod semantic;

pub use models::*;
use semantic::{Importer, Lookup};
use toto_parser::{add_with_loc, ParseError};

pub struct ToscaParser {
    importer: Importer,
}

impl Default for ToscaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ToscaParser {
    pub fn new() -> Self {
        Self {
            importer: Importer::default(),
        }
    }

    pub fn get_files(&self) -> impl Iterator<Item = &url::Url> {
        self.importer.get_files()
    }

    pub fn parse<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file_handle = if let Some(file_handle) = self.importer.get_file(uri) {
            // if !self.importer.is_file_changed(file_handle, ast) {
            //     return Some(file_handle);
            // }
            self.importer.reimport(ast);
            dbg!("REIMPORT!");
            Some(file_handle)
        } else {
            self.importer.add_file(uri, ast)
        };

        let _ = Importer::topo_iter_imports(ast)
            .map_err(|e| {
                add_with_loc(
                    ParseError::Custom("circular import detected".to_string()),
                    e,
                    ast,
                );
            })
            .map(|imports| {
                imports.for_each(|file_handle| {
                    Importer::import_types(file_handle, ast);
                });
            });

        Lookup::lookup(ast);

        file_handle
    }
}
