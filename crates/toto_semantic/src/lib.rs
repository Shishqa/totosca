use std::collections::HashMap;

use petgraph::visit::EdgeRef;

mod errors;

pub use errors::*;
use toto_parser::{grammar::Grammar, tosca::ToscaGrammar};

pub struct File {
    pub source: String,
    pub root: toto_ast::GraphHandle,
}

pub struct Scope {
    pub root: url::Url,
    pub files: HashMap<url::Url, File>,
    pub ast: toto_ast::AST,
}

impl Scope {
    pub fn new() -> Self {
        let pwd = std::env::current_dir().unwrap();
        let root = url::Url::parse(&("file://".to_string() + pwd.to_str().unwrap())).unwrap();

        Self {
            root,
            files: HashMap::new(),
            ast: toto_ast::AST::new(),
        }
    }

    pub fn add_file(&mut self, path: &str) {
        self.add_file_relative(path, self.root.clone());
    }

    pub fn add_file_relative(&mut self, path: &str, root: url::Url) {
        let url = url::Url::parse(path).or(root.join(path));
        match url {
            Ok(url) => {
                if self.files.contains_key(&url) {
                    return;
                }

                let doc = std::fs::read_to_string(url.as_str()[7..].to_string())
                    .map_err(|err| {
                        self.ast.errors.push(Box::new(SemanticError::new(format!(
                            "{}: {}",
                            url.as_str(),
                            err.to_string()
                        ))));
                    })
                    .ok();
                if let None = doc {
                    return;
                }
                let doc = doc.unwrap();

                let root = ToscaGrammar::parse(&doc, &mut self.ast).unwrap();
                let file = File { source: doc, root };
                let imports = file.get_imports(&self.ast);
                self.files.insert(url.clone(), file);

                for import in imports {
                    self.add_file_relative(&import, url.clone());
                }
            }
            Err(error) => {
                self.ast.errors.push(Box::new(SemanticError::new(format!(
                    "invalid url {}: {}",
                    path,
                    error.to_string()
                ))));
            }
        }
    }
}

impl File {
    pub fn get_imports(&self, ast: &toto_ast::AST) -> Vec<String> {
        ast.graph
            .neighbors(self.root)
            .filter(|n| matches!(ast.graph[*n], toto_tosca::Entity::Import))
            .filter_map(|import| {
                ast.graph
                    .edges(import)
                    .filter_map(|e| match &e.weight() {
                        toto_tosca::Relation::Url => Some(e.target()),
                        _ => None,
                    })
                    .find_map(|url| match &ast.graph[url] {
                        toto_tosca::Entity::String(url) => Some(url.to_string()),
                        _ => None,
                    })
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::dot::Dot;

    use crate::Scope;

    #[test]
    fn it_works() {
        let mut scope = Scope::new();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../../tests/a.yaml");

        scope.add_file(d.to_str().unwrap());
        let errors = scope.ast.errors;

        dbg!(Dot::new(&scope.ast.graph));

        if !errors.is_empty() {
            Report::build(ReportKind::Error, d.to_str().unwrap(), 0)
                .with_labels(
                    errors
                        .iter()
                        .map(|err| {
                            let pos: usize = err.loc().try_into().unwrap();
                            Label::new((d.to_str().unwrap(), pos..pos + 1)).with_message(err.what())
                        })
                        .collect::<Vec<_>>(),
                )
                .finish()
                .eprint((
                    d.to_str().unwrap(),
                    Source::from(include_str!("../../../tests/a.yaml")),
                ))
                .unwrap();
        }

        assert!(errors.is_empty());
        assert!(scope.files.len() == 2);
    }
}
