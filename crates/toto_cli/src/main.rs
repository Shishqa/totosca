use std::{env, process::exit};

use clap::Parser;

mod models;
use models::*;
use toto_parser::{get_errors, report_error};
use toto_tosca::ToscaParser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();

    let mut ast = toto_ast::AST::<Entity, Relation>::new();

    let doc_path = "file://".to_string() + env::current_dir().unwrap().to_str().unwrap() + "/";
    let doc_path = url::Url::parse(&doc_path).unwrap();
    let doc_path = doc_path
        .join(&args.path)
        .or(url::Url::parse(&args.path))
        .unwrap();

    let mut parser = ToscaParser::new();
    parser.parse(&doc_path, &mut ast).unwrap();

    let errors = get_errors(&ast).collect::<Vec<_>>();
    let has_errors = !errors.is_empty();

    errors
        .into_iter()
        .for_each(|(what, loc)| report_error(what, loc, &ast));

    exit(if has_errors { 1 } else { 0 });
}
