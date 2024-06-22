use std::{env, error::Error};

use clap::{Parser, Subcommand};

mod models;
use toto_parser::{get_errors, report_error};
use toto_tosca::ToscaParser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum Command {
    /// lint TOSCA file
    ///
    /// This command will report any grammar issues it can find
    /// within provided file and imported files
    Check { path: String },

    /// start language server
    ///
    /// This command will start a TOSCA language server, which
    /// will provide various services to support the development
    /// and validation of TOSCA files. This includes autocompletion,
    /// real-time error checking, enhancing the efficiency and accuracy
    /// of working with TOSCA files
    LS,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    match args.command {
        Command::Check { path } => check(path),
        Command::LS => run_ls(),
    }
}

fn check(path: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ast = toto_ast::AST::<models::Entity, models::Relation>::new();

    let doc_path = "file://".to_string() + env::current_dir().unwrap().to_str().unwrap() + "/";
    let doc_path = url::Url::parse(&doc_path).unwrap();
    let doc_path = doc_path.join(&path).or(url::Url::parse(&path)).unwrap();

    let mut parser = ToscaParser::new();
    parser.parse(&doc_path, &mut ast).unwrap();

    let errors = get_errors(&ast).collect::<Vec<_>>();
    let has_errors = !errors.is_empty();

    errors
        .into_iter()
        .for_each(|(what, loc)| report_error(what, loc, &ast));

    if !has_errors {
        Ok(())
    } else {
        Err("validation failed".into())
    }
}

fn run_ls() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server = toto_lsp::server::Server::new();
    server.run()
}
