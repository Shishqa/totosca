pub mod grammar;
pub mod models;
pub mod semantic;

pub use models::*;
use semantic::{Importer, Lookup};
use toto_parser::{add_with_loc, ParseError};

pub struct ToscaParser {
    importer: Importer,
}

impl ToscaParser {
    pub fn new() -> Self {
        Self {
            importer: Importer::new(),
        }
    }

    pub fn parse<E, R>(
        &mut self,
        uri: &url::Url,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file_handle = self.importer.add_file(uri, ast);

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

        let lookup = Lookup::from_ast(ast);
        lookup.lookup(ast);

        file_handle
    }
}
