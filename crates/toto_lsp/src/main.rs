use std::error::Error;

use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::PrinterContext;
use lsp_types::notification::Notification;

use petgraph::visit::EdgeRef;
use serde_json::from_value;

mod models;

use models::*;
use toto_ast::EntityParser;
use toto_parser::{get_errors, get_yaml_len, AsParseError};
use toto_tosca::grammar::parser::ToscaParser;
use toto_yaml::{AsFileEntity, AsFileRelation};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = lsp_server::Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(lsp_types::ServerCapabilities {
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::INCREMENTAL,
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

fn refresh_diag(
    connection: &lsp_server::Connection,
    uri: &url::Url,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("trying read: {uri:?}");

    let mut ast = toto_ast::AST::<Entity, Relation>::new();

    let doc_handle = ast.add_node(toto_yaml::FileEntity::from_url(uri.clone()).into());
    let doc_root = toto_yaml::YamlParser::parse(doc_handle, &mut ast).unwrap();
    ToscaParser::parse(doc_root, &mut ast);

    let doc = ast.node_weight(doc_handle).unwrap().as_file().unwrap();
    let diagnostics: Vec<lsp_types::Diagnostic> = get_errors(&ast)
        .map(|(what, loc)| {
            let len = get_yaml_len(loc, &ast);
            let pos = ast
                .edges(loc)
                .find_map(|e| match e.weight().as_file() {
                    Some(pos) => Some(pos.0),
                    _ => None,
                })
                .unwrap();

            let get_lc = |offset| -> (u32, u32) {
                let linebreaks = doc.content[0..offset]
                    .chars()
                    .enumerate()
                    .filter_map(|c| if c.1 == '\n' { Some(c.0) } else { None })
                    .collect::<Vec<_>>();
                let lineno = linebreaks.len();
                let charno = pos - linebreaks.iter().next_back().copied().unwrap_or_default() - 1;
                (lineno as u32, charno as u32)
            };

            let (lineno_start, charno_start) = get_lc(pos);
            let (lineno_end, charno_end) = get_lc(pos + len);

            lsp_types::Diagnostic::new(
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
                format!("{:?}", ast.node_weight(what).unwrap().as_parse().unwrap()),
                None,
                None,
            )
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

    connection.sender.send(notif)?;

    Ok(())
}

fn main_loop(
    connection: lsp_server::Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: lsp_types::InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            lsp_server::Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
            }
            lsp_server::Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            lsp_server::Message::Notification(not) => {
                eprintln!("got notification: {not:?}");

                match not.method.as_str() {
                    lsp_types::notification::DidOpenTextDocument::METHOD => {
                        let params: lsp_types::DidOpenTextDocumentParams = from_value(not.params)?;
                        refresh_diag(&connection, &params.text_document.uri)?;
                        continue;
                    }
                    lsp_types::notification::DidSaveTextDocument::METHOD => {
                        let params: lsp_types::DidSaveTextDocumentParams = from_value(not.params)?;
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
