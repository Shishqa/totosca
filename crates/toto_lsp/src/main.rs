use std::error::Error;

use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::PrinterContext;
use lsp_types::notification::Notification;
use lsp_types::{CodeLensOptions, Location};

use lsp_types::request::Request;
use petgraph::visit::EdgeRef;
use petgraph::Direction::{Incoming, Outgoing};
use serde_json::from_value;

mod models;

use models::*;
use toto_ast::EntityParser;
use toto_parser::{get_errors, get_yaml_len, AsParseError, AsParseLoc};
use toto_tosca::grammar::parser::ToscaParser;
use toto_tosca::semantic::Importer;
use toto_tosca::{AsToscaEntity, AsToscaRelation};
use toto_yaml::{AsFileEntity, AsFileRelation};

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
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server_capabilities = serde_json::to_value(lsp_types::ServerCapabilities {
            text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
                lsp_types::TextDocumentSyncKind::INCREMENTAL,
            )),
            definition_provider: Some(lsp_types::OneOf::Left(true)),
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
            .chars()
            .enumerate()
            .filter_map(|c| if c.1 == '\n' { Some(c.0) } else { None })
            .collect::<Vec<_>>();
        let lineno = linebreaks.len();
        let charno = offset - linebreaks.iter().next_back().copied().unwrap_or_default() - 1;
        (lineno as u32, charno as u32)
    }

    fn from_lc(doc: &str, lineno: u32, charno: u32) -> usize {
        let mut curr_lineno = 0;
        let mut curr_charno = 0;
        doc.chars()
            .take_while(|c| {
                if curr_lineno == lineno && curr_charno == charno {
                    return false;
                }
                if *c == '\n' {
                    curr_lineno += 1;
                    curr_charno = 0;
                } else {
                    curr_charno += 1;
                }
                true
            })
            .count()
    }

    fn refresh_diag(&mut self, uri: &url::Url) -> Result<(), Box<dyn Error + Sync + Send>> {
        eprintln!("trying read: {uri:?}");

        self.ast.clear();

        let mut doc = toto_yaml::FileEntity::from_url(uri.clone());
        doc.fetch().unwrap();

        let doc_handle = self.ast.add_node(doc.into());
        let doc_root = toto_yaml::YamlParser::parse(doc_handle, &mut self.ast).unwrap();
        ToscaParser::parse(doc_root, &mut self.ast);

        let doc = self.ast.node_weight(doc_handle).unwrap().as_file().unwrap();
        let diagnostics: Vec<lsp_types::Diagnostic> = get_errors(&self.ast)
            .filter_map(|(what, loc)| {
                let len = get_yaml_len(loc, &self.ast);
                let (pos, file) = self
                    .ast
                    .edges(loc)
                    .find_map(|e| match e.weight().as_file() {
                        Some(pos) => Some((pos.0, e.target())),
                        _ => None,
                    })
                    .unwrap();
                if file != doc_handle {
                    return None;
                }

                let (lineno_start, charno_start) =
                    Self::get_lc(&doc.content.as_ref().unwrap(), pos);
                let (lineno_end, charno_end) =
                    Self::get_lc(&doc.content.as_ref().unwrap(), pos + len);

                Some(lsp_types::Diagnostic::new(
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
                        "{:?}",
                        self.ast.node_weight(what).unwrap().as_parse().unwrap()
                    ),
                    None,
                    None,
                ))
            })
            .collect();

        let notif_params = Some(lsp_types::PublishDiagnosticsParams {
            uri: uri.clone(),
            version: None,
            diagnostics,
        });
        let notif_params = serde_json::to_value(notif_params)?;

        let notif = lsp_server::Message::Notification(lsp_server::Notification {
            method: lsp_types::notification::PublishDiagnostics::METHOD.into(),
            params: notif_params,
        });

        eprintln!("sending: {notif:?}");
        self.connection.sender.send(notif)?;

        Ok(())
    }

    fn goto_definition(
        &self,
        req: &lsp_server::Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let params = from_value::<lsp_types::GotoDefinitionParams>(req.params.clone())?
            .text_document_position_params;

        let existing_urls = Importer::find_urls(&self.ast);
        let file_handle = existing_urls.get(&params.text_document.uri).unwrap();

        let params_pos = Self::from_lc(
            &self
                .ast
                .node_weight(*file_handle)
                .unwrap()
                .as_file()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            params.position.line,
            params.position.character,
        );

        let target_file = self
            .ast
            .edges_directed(*file_handle, Incoming)
            .filter_map(|e| {
                let pos = match e.weight().as_file() {
                    Some(pos) => Some((pos.0, e.source())),
                    _ => None,
                };
                if pos.is_none() {
                    return None;
                }
                let (pos, yaml_handle) = pos.unwrap();
                let len = get_yaml_len(yaml_handle, &self.ast);

                if pos <= params_pos && params_pos <= pos + len {
                    Some(self.ast.edges_directed(yaml_handle, Incoming))
                } else {
                    None
                }
            })
            .flatten()
            .filter_map(|e| match e.weight().as_parse_loc() {
                Some(_) => Some(e.source()),
                _ => None,
            })
            .filter_map(|n| match self.ast.node_weight(n).unwrap().as_tosca() {
                Some(toto_tosca::Entity::Definition) => Some(self.ast.edges_directed(n, Outgoing)),
                _ => None,
            })
            .flatten()
            .find_map(|e| match dbg!(e.weight()).as_tosca() {
                Some(toto_tosca::Relation::ImportFile) => Some(e.target()),
                _ => None,
            });

        if target_file.is_none() {
            eprintln!("couldn't find target file");
            return Ok(());
        }

        let file = self
            .ast
            .edges_directed(target_file.unwrap(), Outgoing)
            .filter_map(|e| match e.weight().as_parse_loc() {
                Some(_) => Some(self.ast.edges_directed(e.target(), Outgoing)),
                _ => None,
            })
            .flatten()
            .find_map(|e| match e.weight().as_file() {
                Some(_) => Some(e.target()),
                _ => None,
            })
            .map(|file_handle| {
                self.ast
                    .node_weight(file_handle)
                    .unwrap()
                    .as_file()
                    .unwrap()
            })
            .unwrap();

        let response = lsp_types::GotoDefinitionResponse::from(Location::new(
            file.url.clone(),
            lsp_types::Range::new(
                lsp_types::Position::new(0, 0),
                lsp_types::Position::new(0, 1),
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
