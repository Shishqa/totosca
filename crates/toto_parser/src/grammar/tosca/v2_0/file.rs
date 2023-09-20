use toto_tosca::{Entity, Relation};

use super::{
    parse_collection, parse_list, DataType, ImportDefinition, NodeType, ServiceTemplateDefinition,
};
use crate::parse::{Error, GraphHandle, Parse, ParseError};

#[derive(Debug)]
pub struct ToscaFileDefinition;

impl Parse for ToscaFileDefinition {
    fn parse(ctx: &mut crate::parse::Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::File);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "tosca_definitions_version" => {}
                    "profile" => {
                        let t = String::parse(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Profile);
                    }
                    "metadata" => {
                        parse_collection::<String>(ctx, root, entry.1);
                    }
                    "description" => {
                        let t = String::parse(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "imports" => {
                        parse_list::<ImportDefinition>(ctx, root, entry.1);
                    }
                    "data_types" => {
                        parse_collection::<DataType>(ctx, root, entry.1);
                    }
                    "node_types" => {
                        parse_collection::<NodeType>(ctx, root, entry.1);
                    }
                    "service_template" => {
                        let t = ServiceTemplateDefinition::parse(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::ServiceTemplate);
                    }
                    f => ctx.errors.push(Error {
                        pos: Some(entry.0.pos()),
                        error: ParseError::UnknownField(f.to_string()),
                    }),
                });
        } else {
            ctx.errors.push(Error {
                pos: Some(n.pos()),
                error: ParseError::UnexpectedType("map"),
            });
            return root;
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::dot::Dot;

    use super::*;
    use crate::parse::parse;

    #[test]
    fn it_works() {
        const DOC: &str = include_str!("../../../tests/file.yaml");

        dbg!(Dot::new(
            &parse::<ToscaFileDefinition>(DOC)
                .map_err(|errors| {
                    Report::build(ReportKind::Error, "../../../tests/file.yaml", 0)
                        .with_labels(
                            errors
                                .iter()
                                .map(|err| {
                                    let pos: usize =
                                        err.pos.unwrap_or_default().try_into().unwrap();
                                    Label::new(("../../../tests/file.yaml", pos..pos + 1))
                                        .with_message(format!("{:?}", err.error))
                                })
                                .collect::<Vec<_>>(),
                        )
                        .finish()
                        .eprint(("../../../tests/file.yaml", Source::from(DOC)))
                        .unwrap();
                })
                .unwrap()
        ));
    }
}
