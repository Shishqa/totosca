use lsp_types::notification::Notification;
use lsp_types::request::Request;
use serde_json::from_value;
use std::error::Error;

use crate::{capabilities, models};

pub struct Server {
    connection: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,
    ast: toto_ast::AST<models::Entity, models::Relation>,
    parser: toto_tosca::ToscaParser,
}

impl Server {
    pub fn new() -> Self {
        let (connection, io_threads) = lsp_server::Connection::stdio();
        Self {
            connection,
            io_threads,
            ast: toto_ast::AST::<models::Entity, models::Relation>::new(),
            parser: toto_tosca::ToscaParser::default(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server_capabilities = serde_json::to_value(lsp_types::ServerCapabilities {
            text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
                lsp_types::TextDocumentSyncKind::NONE,
            )),
            definition_provider: Some(lsp_types::OneOf::Left(true)),
            completion_provider: Some(lsp_types::CompletionOptions {
                trigger_characters: Some(vec![": ".to_string(), "  ".to_string()]),
                ..Default::default()
            }),
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

    fn refresh_diag(&mut self, uri: &url::Url) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut diagnostics =
            capabilities::diagnostics::get_diagnostics(&mut self.parser, &mut self.ast, uri)?;

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

    fn completion(
        &mut self,
        req: &lsp_server::Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let params =
            from_value::<lsp_types::CompletionParams>(req.params.clone())?.text_document_position;

        self.refresh_diag(&params.text_document.uri)?;

        let suggests = capabilities::complete::complete_at(
            &mut self.ast,
            &params.text_document.uri,
            params.position.line,
            params.position.character,
        );

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
        &mut self,
        req: &lsp_server::Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let params = from_value::<lsp_types::GotoDefinitionParams>(req.params.clone())?
            .text_document_position_params;

        self.refresh_diag(&params.text_document.uri)?;

        let Some(location) = capabilities::goto_definition::goto_definition(
            &mut self.ast,
            &params.text_document.uri,
            params.position.line,
            params.position.character,
        ) else {
            return Ok(());
        };

        let response = lsp_types::GotoDefinitionResponse::from(location);
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
