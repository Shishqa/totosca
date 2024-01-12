use toto_tosca::{Entity, Relation};

use super::{parse_collection, parse_list};
use crate::{
    parse::{Error, GraphHandle, ParseError},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ToscaFileDefinition;

impl Parse for ToscaFileDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut crate::parse::Context,
        n: &yaml_peg::NodeRc,
    ) -> GraphHandle {
        let root = ctx.graph.add_node(Entity::File);

        if let Ok(map) = n.as_map() {
            map.iter()
                .for_each(|entry| match entry.0.as_str().unwrap() {
                    "tosca_definitions_version" => {}
                    "profile" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Profile);
                    }
                    "metadata" => {
                        parse_collection::<String, V>(ctx, root, entry.1);
                    }
                    "description" => {
                        let t = String::parse::<V>(ctx, entry.1);
                        ctx.graph.add_edge(root, t, Relation::Description);
                    }
                    "imports" => {
                        parse_list::<V::ImportDefinition, V>(ctx, root, entry.1);
                    }
                    "data_types" => {
                        parse_collection::<V::DataTypeDefinition, V>(ctx, root, entry.1);
                    }
                    "node_types" => {
                        parse_collection::<V::NodeTypeDefinition, V>(ctx, root, entry.1);
                    }
                    "service_template" => {
                        let t = V::ServiceTemplateDefinition::parse::<V>(ctx, entry.1);
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

    use crate::{parse::parse, tosca::ToscaGrammar};

    #[test]
    fn it_works() {
        const DOC: &str = include_str!("../../tests/tosca_2_0.yaml");

        dbg!(Dot::new(
            &parse::<ToscaGrammar>(DOC)
                .map_err(|errors| {
                    Report::build(ReportKind::Error, "../../tests/tosca_2_0.yaml", 0)
                        .with_labels(
                            errors
                                .iter()
                                .map(|err| {
                                    let pos: usize =
                                        err.pos.unwrap_or_default().try_into().unwrap();
                                    Label::new(("../../tests/tosca_2_0.yaml", pos..pos + 1))
                                        .with_message(format!("{:?}", err.error))
                                })
                                .collect::<Vec<_>>(),
                        )
                        .finish()
                        .eprint(("../../tests/tosca_2_0.yaml", Source::from(DOC)))
                        .unwrap();
                })
                .unwrap()
        ));
    }
}
