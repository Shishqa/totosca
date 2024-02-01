use toto_tosca::{Entity, Relation};

use super::{parse_collection, parse_list};
use crate::{
    parse::{ParseError, ParseErrorKind},
    tosca::{Parse, ToscaDefinitionsVersion},
};

#[derive(Debug)]
pub struct ToscaFileDefinition;

impl Parse for ToscaFileDefinition {
    fn parse<V: ToscaDefinitionsVersion>(
        ctx: &mut toto_ast::AST,
        n: &yaml_peg::NodeRc,
    ) -> toto_ast::GraphHandle {
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
                    f => ctx.errors.push(Box::new(ParseError {
                        pos: Some(entry.0.pos()),
                        error: ParseErrorKind::UnknownField(f.to_string()),
                    })),
                });
        } else {
            ctx.errors.push(Box::new(ParseError {
                pos: Some(n.pos()),
                error: ParseErrorKind::UnexpectedType("map"),
            }));
            return root;
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::dot::Dot;

    use crate::{grammar::Grammar, tosca::ToscaGrammar};

    #[test]
    fn it_works() {
        let doc = include_str!("../../tests/tosca_2_0.yaml");

        let mut ast = toto_ast::AST::new();

        ToscaGrammar::parse(doc, &mut ast);
        let errors = ast.errors;

        dbg!(Dot::new(&ast.graph));

        if !errors.is_empty() {
            Report::build(ReportKind::Error, "../../tests/tosca_2_0.yaml", 0)
                .with_labels(
                    errors
                        .iter()
                        .map(|err| {
                            let pos: usize = err.loc().try_into().unwrap();
                            Label::new(("../../tests/tosca_2_0.yaml", pos..pos + 1))
                                .with_message(err.what())
                        })
                        .collect::<Vec<_>>(),
                )
                .finish()
                .eprint((
                    "../../tests/tosca_2_0.yaml",
                    Source::from(include_str!("../../tests/tosca_2_0.yaml")),
                ))
                .unwrap();
        }

        assert!(errors.is_empty())
    }
}
