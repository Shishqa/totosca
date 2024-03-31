use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use lsp_types::notification::Notification;
use lsp_types::{CompletionItemKind, Location};

use lsp_types::request::Request;
use petgraph::data::DataMap;
use petgraph::dot::Dot;
use petgraph::visit::EdgeRef;
use petgraph::Direction::{self, Incoming, Outgoing};
use serde_json::from_value;

mod models;

use models::*;
use toto_parser::{get_errors, get_yaml_len, AsParseError, AsParseLoc};
use toto_tosca::{AsToscaRelation, ImportTargetRelation};
use toto_yaml::{AsFileEntity, AsFileRelation, AsYamlEntity};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    let server = Server::new();
    server.run()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

struct Server {
    connection: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,
    ast: toto_ast::AST<Entity, Relation>,
    parser: toto_tosca::ToscaParser,
}

impl Server {
    pub fn new() -> Self {
        // Create the transport. Includes the stdio (stdin and stdout) versions but this could
        // also be implemented to use sockets or HTTP.
        let (connection, io_threads) = lsp_server::Connection::stdio();
        Self {
            connection,
            io_threads,
            ast: toto_ast::AST::<Entity, Relation>::new(),
            parser: toto_tosca::ToscaParser::default(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server_capabilities = serde_json::to_value(lsp_types::ServerCapabilities {
            text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
                lsp_types::TextDocumentSyncKind::INCREMENTAL,
            )),
            definition_provider: Some(lsp_types::OneOf::Left(true)),
            completion_provider: Some(lsp_types::CompletionOptions {
                trigger_characters: Some(vec![": ".to_string()]),
                ..Default::default()
            }),
            // diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            //     identifier: None,
            //     inter_file_dependencies: false,
            //     workspace_diagnostics: false,
            //     work_done_progress_options: WorkDoneProgressOptions {
            //         work_done_progress: None,
            //     },
            // })),
            ..Default::default()
        })
        .unwrap();
        let initialization_params = match self.connection.initialize(server_capabilities) {
            Ok(it) => it,
            Err(e) => {
                if e.channel_is_disconnected() {
                    self.io_threads.join()?;
                }
                return Err(e.into());
            }
        };
        let _params: lsp_types::InitializeParams = serde_json::from_value(initialization_params)?;
        eprintln!("starting example main loop");
        for msg in &self.connection.receiver.clone() {
            eprintln!("got msg: {msg:?}");
            match msg {
                lsp_server::Message::Request(req) => {
                    if self.connection.handle_shutdown(&req)? {
                        return Ok(());
                    }

                    eprintln!("got request: {req:?}");
                    match req.method.as_str() {
                        lsp_types::request::GotoDefinition::METHOD => {
                            self.goto_definition(&req)?;
                            continue;
                        }
                        lsp_types::request::Completion::METHOD => {
                            self.completion(&req)?;
                            continue;
                        }
                        &_ => {}
                    }
                }
                lsp_server::Message::Response(resp) => {
                    eprintln!("got response: {resp:?}");
                }
                lsp_server::Message::Notification(not) => {
                    eprintln!("got notification: {not:?}");

                    match not.method.as_str() {
                        lsp_types::notification::DidOpenTextDocument::METHOD => {
                            let params: lsp_types::DidOpenTextDocumentParams =
                                from_value(not.params)?;
                            self.refresh_diag(&params.text_document.uri)?;
                            continue;
                        }
                        lsp_types::notification::DidSaveTextDocument::METHOD => {
                            let params: lsp_types::DidSaveTextDocumentParams =
                                from_value(not.params)?;
                            self.refresh_diag(&params.text_document.uri)?;
                            continue;
                        }
                        &_ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn get_lc(doc: &str, offset: usize) -> (u32, u32) {
        let linebreaks = doc[0..offset]
            .char_indices()
            .filter_map(|c| if c.1 == '\n' { Some(c.0) } else { None })
            .collect::<Vec<_>>();
        let lineno = linebreaks.len();
        let charno = offset - linebreaks.iter().next_back().copied().unwrap_or_default() - 1;
        (lineno as u32, charno as u32)
    }

    fn from_lc(doc: &str, lineno: u32, charno: u32) -> usize {
        let base = doc
            .split_inclusive('\n')
            .take(lineno as usize)
            .fold(0, |acc, l| acc + l.len());

        dbg!(base, charno);
        base + charno as usize
    }

    fn refresh_diag(&mut self, uri: &url::Url) -> Result<(), Box<dyn Error + Sync + Send>> {
        eprintln!("trying read: {uri:?}");

        self.parser.parse(uri, &mut self.ast);

        let mut diagnostics = HashMap::<url::Url, Vec<lsp_types::Diagnostic>>::new();

        let dot = Dot::new(&self.ast);
        let mut file = File::create(".toto-ast.dot")?;
        file.write_all(format!("{:?}", dot).as_bytes())?;

        get_errors(&self.ast).for_each(|(what, loc)| {
            let len = loc.map(|l| get_yaml_len(l, &self.ast)).unwrap_or(1);
            let (pos, file) = self
                .ast
                .edges(what)
                .find_map(|e| e.weight().as_file().map(|pos| (pos.0, e.target())))
                .unwrap();

            let doc = self.ast[file].as_file().unwrap();

            let (lineno_start, charno_start) = Self::get_lc(doc.content.as_ref().unwrap(), pos);
            let (lineno_end, charno_end) = Self::get_lc(doc.content.as_ref().unwrap(), pos + len);

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
                    format!(
                        "{:?} [{},{}] [({},{}),({},{})]",
                        self.ast.node_weight(what).unwrap().as_parse().unwrap(),
                        pos,
                        pos + len,
                        lineno_start,
                        charno_start,
                        lineno_end,
                        charno_end,
                    ),
                    None,
                    None,
                ));
        });

        for uri in self.parser.get_files() {
            let notif_params = Some(lsp_types::PublishDiagnosticsParams {
                uri: uri.clone(),
                version: None,
                diagnostics: diagnostics.remove(uri).unwrap_or_default(),
            });
            let notif_params = serde_json::to_value(notif_params)?;

            let notif = lsp_server::Message::Notification(lsp_server::Notification {
                method: lsp_types::notification::PublishDiagnostics::METHOD.into(),
                params: notif_params,
            });

            eprintln!("sending: {notif:?}");
            self.connection.sender.send(notif)?;
        }

        Ok(())
    }

    fn completion(&self, req: &lsp_server::Request) -> Result<(), Box<dyn Error + Sync + Send>> {
        let params =
            from_value::<lsp_types::CompletionParams>(req.params.clone())?.text_document_position;

        eprintln!("looking for {}", params.text_document.uri);
        let file_handle = self
            .ast
            .node_indices()
            .find(|n| matches!(self.ast.node_weight(*n).unwrap().as_file(), Some(f) if f.url == params.text_document.uri))
            .unwrap();

        let params_pos = Self::from_lc(
            self.ast
                .node_weight(file_handle)
                .unwrap()
                .as_file()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            params.position.line,
            params.position.character,
        );

        let lookuper = self
            .ast
            .edges_directed(file_handle, Incoming)
            .filter_map(|e| e.weight().as_file().map(|pos| (pos.0, e.source())))
            .filter_map(
                |(pos, source)| match self.ast.node_weight(source).unwrap().as_yaml() {
                    Some(_) => Some((pos, get_yaml_len(source, &self.ast), source)),
                    _ => None,
                },
            )
            .filter_map(|(pos, len, source)| {
                if pos <= params_pos && params_pos <= pos + len {
                    Some(self.ast.edges_directed(source, Incoming))
                } else {
                    None
                }
            })
            .flatten()
            .find_map(|e| match e.weight().as_tosca() {
                Some(toto_tosca::Relation::Ref(referencer)) => {
                    Some((e.id(), referencer.lookuper.clone()))
                }
                _ => None,
            });

        if lookuper.is_none() {
            eprintln!("can't provide completion (no semantic)");
            return Ok(());
        }
        let lookuper = lookuper.unwrap();

        let suggests = lookuper
            .1
            .lookup_suggests(&self.ast, lookuper.0)
            .into_iter()
            .map(|(name, detail, rel)| {
                let mut item =
                    lsp_types::CompletionItem::new_simple(name, detail.unwrap_or_default());

                item.kind = match rel {
                    toto_tosca::Relation::Type(_) => Some(CompletionItemKind::TYPE_PARAMETER),
                    toto_tosca::Relation::Definition(_) => Some(CompletionItemKind::REFERENCE),
                    _ => None,
                };

                item
            })
            .collect::<Vec<_>>();

        let response = lsp_types::CompletionResponse::Array(suggests);
        let response = serde_json::to_value(response)?;

        let response = lsp_server::Message::Response(lsp_server::Response {
            id: req.id.clone(),
            result: Some(response),
            error: None,
        });

        eprintln!("sending: {response:?}");
        self.connection.sender.send(response)?;

        Ok(())
    }

    fn goto_definition(
        &self,
        req: &lsp_server::Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let params = from_value::<lsp_types::GotoDefinitionParams>(req.params.clone())?
            .text_document_position_params;

        eprintln!("looking for {}", params.text_document.uri);
        let file_handle = self
            .ast
            .node_indices()
            .find(|n| matches!(self.ast.node_weight(*n).unwrap().as_file(), Some(f) if f.url == params.text_document.uri))
            .unwrap();

        let params_pos = Self::from_lc(
            self.ast
                .node_weight(file_handle)
                .unwrap()
                .as_file()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            params.position.line,
            params.position.character,
        );
        dbg!(params.position.line, params.position.character, params_pos);

        let semantic_token = self
            .ast
            .edges_directed(file_handle, Incoming)
            .filter_map(|e| e.weight().as_file().map(|pos| (pos.0, e.source())))
            .filter_map(
                |(pos, source)| match self.ast.node_weight(source).unwrap().as_yaml() {
                    Some(_) => Some((pos, get_yaml_len(source, &self.ast), source)),
                    _ => None,
                },
            )
            .filter_map(|(pos, len, source)| {
                if pos <= params_pos && params_pos <= pos + len {
                    Some(self.ast.edges_directed(source, Incoming))
                } else {
                    None
                }
            })
            .flatten()
            .find_map(|e| match e.weight().as_tosca() {
                Some(toto_tosca::Relation::Ref(referencer)) => {
                    Some((e.source(), referencer.lookuper.then.clone()))
                }
                Some(toto_tosca::Relation::ImportUrl(_)) => {
                    Some((e.source(), toto_tosca::Relation::from(ImportTargetRelation)))
                }
                _ => None,
            });

        if semantic_token.is_none() {
            eprintln!("can't go to definition (no semantic) {}", params_pos);
            return Ok(());
        }
        let semantic_token = semantic_token.unwrap();

        let goto_target = self
            .ast
            .edges_directed(semantic_token.0, Outgoing)
            .find_map(|e| {
                if e.weight().as_tosca() == Some(&semantic_token.1) {
                    Some(e.target())
                } else {
                    None
                }
            });

        if goto_target.is_none() {
            eprintln!("can't go to definition (no target)");
            return Ok(());
        }
        let goto_target = goto_target.unwrap();

        let (target_file, target_pos) = self
            .ast
            .edges_directed(goto_target, Outgoing)
            .filter_map(|e| match e.weight().as_parse_loc() {
                Some(_) => Some(self.ast.edges_directed(e.target(), Outgoing)),
                _ => None,
            })
            .flatten()
            .find_map(|e| e.weight().as_file().map(|loc| (e.target(), loc.0)))
            .map(|(file_handle, pos)| {
                let file = self
                    .ast
                    .node_weight(file_handle)
                    .unwrap()
                    .as_file()
                    .unwrap();
                (file, pos)
            })
            .unwrap();

        let (target_l, target_c) = Self::get_lc(target_file.content.as_ref().unwrap(), target_pos);

        let response = lsp_types::GotoDefinitionResponse::from(Location::new(
            target_file.url.clone(),
            lsp_types::Range::new(
                lsp_types::Position::new(target_l, target_c),
                lsp_types::Position::new(target_l, target_c + 1),
            ),
        ));
        let response = serde_json::to_value(response)?;

        let response = lsp_server::Message::Response(lsp_server::Response {
            id: req.id.clone(),
            result: Some(response),
            error: None,
        });

        eprintln!("sending: {response:?}");
        self.connection.sender.send(response)?;

        Ok(())
    }
}
