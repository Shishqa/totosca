use ariadne::{Label, Report, ReportKind, Source};
use petgraph::visit::EdgeRef;

use crate::{ParseCompatibleEntity, ParseCompatibleRelation};

pub fn get_errors<E, R>(
    ast: &toto_ast::AST<E, R>,
) -> impl Iterator<Item = (toto_ast::GraphHandle, Option<toto_ast::GraphHandle>)>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    ast.node_indices()
        .into_iter()
        .filter_map(|node| match ast[node].as_parse() {
            Some(_) => Some(node),
            _ => None,
        })
        .map(|node| {
            let yaml = ast
                .edges(node)
                .find_map(|e| match e.weight().as_parse_loc() {
                    Some(_) => Some(e.target()),
                    _ => None,
                });
            (node, yaml)
        })
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn report_error<E, R>(
    what: toto_ast::GraphHandle,
    loc: Option<toto_ast::GraphHandle>,
    ast: &toto_ast::AST<E, R>,
) where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    let len = loc.map(|l| get_yaml_len(l, ast)).unwrap_or(1);
    let (pos, file) = ast
        .edges(what)
        .find_map(|e| match e.weight().as_file() {
            Some(pos) => Some((pos.0, e.target())),
            _ => None,
        })
        .unwrap();

    let err = ast.node_weight(what).unwrap().as_parse().unwrap();
    let file = ast.node_weight(file).unwrap().as_file().unwrap();

    Report::build(ReportKind::Error, file.url.as_str(), pos)
        .with_label(
            Label::new((file.url.as_str(), pos..pos + len)).with_message(format!("{:?}", err)),
        )
        .finish()
        .eprint((
            file.url.as_str(),
            Source::from(file.content.as_ref().unwrap().as_str()),
        ))
        .unwrap();
}

pub fn get_yaml_len<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> usize
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    match ast.node_weight(n).unwrap().as_yaml() {
        Some(toto_yaml::Entity::Int(n)) => (n.checked_ilog10().unwrap_or(0) + 1) as usize,
        Some(toto_yaml::Entity::Float(n)) => n.to_string().chars().count(),
        Some(toto_yaml::Entity::Str(s)) => s.len(),
        _ => 1 as usize,
    }
}
