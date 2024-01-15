//! A minimal example LSP server that can only respond to the `gotoDefinition` request. To use
//! this example, execute it and then send an `initialize` request.
//!
//! ```no_run
//! Content-Length: 85
//!
//! {"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"capabilities": {}}}
//! ```
//!
//! This will respond with a server response. Then send it a `initialized` notification which will
//! have no response.
//!
//! ```no_run
//! Content-Length: 59
//!
//! {"jsonrpc": "2.0", "method": "initialized", "params": {}}
//! ```
//!
//! Once these two are sent, then we enter the main loop of the server. The only request this
//! example can handle is `gotoDefinition`:
//!
//! ```no_run
//! Content-Length: 159
//!
//! {"jsonrpc": "2.0", "method": "textDocument/definition", "id": 2, "params": {"textDocument": {"uri": "file://temp"}, "position": {"line": 1, "character": 1}}}
//! ```
//!
//! To finish up without errors, send a shutdown request:
//!
//! ```no_run
//! Content-Length: 67
//!
//! {"jsonrpc": "2.0", "method": "shutdown", "id": 3, "params": null}
//! ```
//!
//! The server will exit the main loop and finally we send a `shutdown` notification to stop
//! the server.
//!
//! ```
//! Content-Length: 54
//!
//! {"jsonrpc": "2.0", "method": "exit", "params": null}
//! ```
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use lsp_types::notification::{
    DidChangeTextDocument, DidOpenTextDocument, DidSaveTextDocument, Notification,
    PublishDiagnostics,
};
use lsp_types::request::{DocumentDiagnosticRequest, Request};
use lsp_types::{
    lsp_notification, lsp_request, Diagnostic, DiagnosticOptions, DiagnosticServerCapabilities,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportKind,
    FullDocumentDiagnosticReport, OneOf, Position, PublishDiagnosticsParams,
    RelatedFullDocumentDiagnosticReport, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    WorkDoneProgressOptions,
};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use lsp_server::{Connection, ExtractError, Message, Response};
use serde_json::from_value;
use toto_parser::tosca::ToscaGrammar;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
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
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn refresh_diag(connection: &Connection, uri: &Url) -> Result<(), Box<dyn Error + Sync + Send>> {
    let path = String::from(uri.clone())[7..].to_string();
    eprintln!("trying read: {path:?}");

    let contents = fs::read_to_string(path)?;

    let result = toto_parser::parse::parse::<ToscaGrammar>(contents.as_str());

    let diagnostics: Vec<Diagnostic> = match result {
        Ok(_) => {
            vec![]
        }
        Err(errors) => errors
            .iter()
            .map(|err| {
                let offset: usize = err.pos.unwrap_or_default().try_into().unwrap();
                let linebreaks = contents[0..offset]
                    .chars()
                    .enumerate()
                    .filter_map(|c| if c.1 == '\n' { Some(c.0) } else { None })
                    .collect::<Vec<_>>();
                let lineno = linebreaks.len();
                let charno = offset
                    - linebreaks
                        .iter()
                        .rev()
                        .next()
                        .map(|n| *n)
                        .unwrap_or_default()
                    - 1;

                Diagnostic::new(
                    lsp_types::Range {
                        start: Position {
                            line: lineno as u32,
                            character: charno as u32,
                        },
                        end: Position {
                            line: lineno as u32,
                            character: charno as u32 + 1,
                        },
                    },
                    None,
                    None,
                    None,
                    format!("{:?}", err.error),
                    None,
                    None,
                )
            })
            .collect(),
    };

    let notif_params = Some(PublishDiagnosticsParams {
        uri: uri.clone(),
        version: None,
        diagnostics,
    });
    let notif_params = serde_json::to_value(&notif_params)?;

    let notif = Message::Notification(lsp_server::Notification {
        method: PublishDiagnostics::METHOD.into(),
        params: notif_params,
    });

    eprintln!("sending: {notif:?}");

    connection.sender.send(notif)?;

    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");

                match not.method.as_str() {
                    DidOpenTextDocument::METHOD => {
                        let params: DidOpenTextDocumentParams = from_value(not.params)?;
                        refresh_diag(&connection, &params.text_document.uri)?;
                        continue;
                    }
                    DidSaveTextDocument::METHOD => {
                        let params: DidSaveTextDocumentParams = from_value(not.params)?;
                        refresh_diag(&connection, &params.text_document.uri)?;
                        continue;
                    }
                    &_ => {}
                }
            }
        }
    }
    Ok(())
}
