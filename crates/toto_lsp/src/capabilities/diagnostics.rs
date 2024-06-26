use std::{collections::HashMap, error::Error};

use petgraph::visit::EdgeRef;
use toto_parser::AsParseError;
use toto_yaml::{AsFileEntity, AsFileRelation};

use crate::models;

pub(crate) fn get_diagnostics(
    parser: &mut toto_tosca::ToscaParser,
    ast: &mut toto_ast::AST<models::Entity, models::Relation>,
    uri: &url::Url,
) -> Result<HashMap<url::Url, Vec<lsp_types::Diagnostic>>, Box<dyn Error + Sync + Send>> {
    eprintln!("trying read: {uri:?}");

    parser.parse(uri, ast)?;

    let mut diagnostics = HashMap::<url::Url, Vec<lsp_types::Diagnostic>>::new();

    toto_parser::get_errors(&ast).for_each(|(what, loc)| {
        let len = loc.map_or(1, |l| toto_parser::get_yaml_len(l, &ast));
        let (pos, file) = ast
            .edges(what)
            .find_map(|e| e.weight().as_file().map(|pos| (pos.0, e.target())))
            .unwrap();

        let doc = ast[file].as_file().unwrap();

        let (lineno_start, charno_start) = toto_yaml::get_lc(doc.content.as_ref().unwrap(), pos);
        let (lineno_end, charno_end) = toto_yaml::get_lc(doc.content.as_ref().unwrap(), pos + len);

        if !diagnostics.contains_key(&doc.url) {
            diagnostics.insert(doc.url.clone(), vec![]);
        }

        diagnostics
            .get_mut(&doc.url)
            .unwrap()
            .push(lsp_types::Diagnostic::new(
                lsp_types::Range {
                    start: lsp_types::Position {
                        line: lineno_start,
                        character: charno_start,
                    },
                    end: lsp_types::Position {
                        line: lineno_end,
                        character: charno_end,
                    },
                },
                None,
                None,
                None,
                format!("{}", ast.node_weight(what).unwrap().as_parse().unwrap()),
                None,
                None,
            ));
    });

    Ok(diagnostics)
}
