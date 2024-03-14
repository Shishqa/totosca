use std::marker::PhantomData;

use crate::{
    parse::{
        add_error, parse_schema, EntityParser, ParseError, ParseLoc, RelationParser, Schema,
        StaticSchemaMap,
    },
    tosca::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion},
};

use super::{
    import,
    value::{self, Description, Field},
};

#[derive(Debug)]
pub struct ToscaFileDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> Schema<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion,
{
    const SCHEMA: StaticSchemaMap<E, R> = phf::phf_map! {
        // "tosca_definitions_version" => |r, n, ast| {
        //     let t =
        //         ast.add_node(toto_tosca::Entity::ToscaDefinitionsVersion.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "profile" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Profile.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "metadata" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Metadata.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        "description" => value::Field::<value::Description, value::String>::parse::<E, R>,
        "imports" => value::List::<import::Import, V::ImportDefinition>::parse::<E, R>,
        // "imports" => |r, n, ast| {
        //     List::<E, R, V::ImportDefinition>(n, r, PhantomData::default())
        //         .parse(ast);
        // },
        // "data_types" => |r, n, ast| {
        //     Collection::<E, R, V::DataTypeDefinition>(
        //         n,
        //         r,
        //         PhantomData::default(),
        //     )
        //     .parse(ast);
        // },
        // "node_types" => |r, n, ast| {
        //     Collection::<E, R, V::NodeTypeDefinition>(
        //         n,
        //         r,
        //         PhantomData::default(),
        //     )
        //     .parse(ast);
        // },
        // "service_template" => |r, n, ast| {
        //     let t = V::ServiceTemplateDefinition::from(n).parse(ast);
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        // }
    };
}

impl<V> EntityParser<toto_ast::GraphHandle> for ToscaFileDefinition<V>
where
    V: ToscaDefinitionsVersion,
{
    fn parse<E, R>(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file = ast.add_node(toto_tosca::Entity::File.into());
        toto_yaml::as_map(n, ast)
            .or_else(|| {
                add_error(n, ast, ParseError::UnexpectedType("map"));
                None
            })
            .and_then(|items| {
                parse_schema(&Self::SCHEMA, file, items, ast);
                Some(file)
            });
        Some(file)
    }
}

#[cfg(test)]
mod tests {
    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::{dot::Dot, visit::EdgeRef};

    use crate::{
        parse::ParseError,
        tosca::{ast::ToscaParser, tests::Entity, tests::Relation},
    };

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../../tests/tosca_2_0.yaml");

        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        // let ast = NodeFiltered::from_fn(&ast, |n| matches!(ast[n], Entity::Tosca(_)));

        ToscaParser::parse(doc, &mut ast);

        dbg!(Dot::new(&ast));

        let errors = ast
            .node_indices()
            .into_iter()
            .filter_map(|node| match &ast[node] {
                Entity::Parse(err) => Some((node, err)),
                _ => None,
            })
            .map(|(node, err)| {
                let yaml = ast
                    .edges(node)
                    .find_map(|e| match e.weight() {
                        Relation::Parse(_) => Some(e.target()),
                        _ => None,
                    })
                    .unwrap();
                (yaml, err)
            })
            .map(|(yaml, err)| {
                let pos = ast
                    .edges(yaml)
                    .find_map(|e| match e.weight() {
                        Relation::File(pos) => Some(pos.0),
                        _ => None,
                    })
                    .unwrap();
                (pos, err)
            })
            .collect::<Vec<(usize, &ParseError)>>();

        if !errors.is_empty() {
            Report::build(ReportKind::Error, "../../../../../tests/tosca_2_0.yaml", 0)
                .with_labels(
                    errors
                        .iter()
                        .map(|err| {
                            Label::new(("../../../../../tests/tosca_2_0.yaml", err.0..err.0 + 1))
                                .with_message(format!("{:?}", err.1))
                        })
                        .collect::<Vec<_>>(),
                )
                .finish()
                .eprint((
                    "../../../../../tests/tosca_2_0.yaml",
                    Source::from(include_str!("../../../../../tests/tosca_2_0.yaml")),
                ))
                .unwrap();
        }

        assert!(errors.is_empty())
    }
}
