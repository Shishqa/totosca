use std::marker::PhantomData;

use petgraph::{
    data::{Build, Create, DataMap},
    visit::{Data, EdgeRef, GraphBase, IntoEdgeReferences, IntoEdges},
};

use crate::{
    parse::{ParseError, ParseLoc},
    tosca::{
        AsYamlEntity, AsYamlRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
        ToscaDefinitionsVersion,
    },
};

#[derive(Debug)]
pub struct ToscaFileDefinition<E, R, V>(pub toto_ast::GraphHandle, PhantomData<(V, E, R)>)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>;

impl<E, R, V> From<toto_ast::GraphHandle> for ToscaFileDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn from(value: toto_ast::GraphHandle) -> Self {
        Self(value, PhantomData::default())
    }
}

impl<E, R, V> toto_ast::Parse<E, R> for ToscaFileDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        if let Ok(_) = t.0.as_map() {
            let root = ast.add_node(toto_tosca::Entity::File.into());
            ast.add_edge(root, self.0, ParseLoc.into());

            let keys = ast
                .edges(self.0)
                .filter_map(|e| match e.weight().as_yaml().unwrap() {
                    toto_yaml::Relation::MapKey => Some(e.target()),
                    _ => None,
                })
                .filter_map(
                    |k| match ast.node_weight(k).unwrap().as_yaml().unwrap().0.yaml() {
                        yaml_peg::Yaml::Str(str_key) => Some((k.clone(), str_key.clone())),
                        _ => None,
                    },
                )
                .map(|(k, str_key)| {
                    let v = ast
                        .edges(k)
                        .find_map(|e| match e.weight().as_yaml().unwrap() {
                            toto_yaml::Relation::MapValue => Some(e.target()),
                            _ => None,
                        })
                        .unwrap();

                    (str_key, k, v)
                })
                .collect::<Vec<(String, toto_ast::GraphHandle, toto_ast::GraphHandle)>>();

            keys.iter()
                .for_each(|(str_key, k, v)| match str_key.as_str() {
                    "tosca_definitions_version" => {
                        let t = ast.add_node(toto_tosca::Entity::ToscaDefinitionsVersion.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    "profile" => {
                        let t = ast.add_node(toto_tosca::Entity::Profile.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    "metadata" => {
                        let t = ast.add_node(toto_tosca::Entity::Metadata.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    "description" => {
                        let t = ast.add_node(toto_tosca::Entity::Description.into());
                        ast.add_edge(root, t, toto_tosca::Relation::Subdef.into());
                        ast.add_edge(t, *v, ParseLoc.into());
                    }
                    // "imports" => {
                    //     parse_list::<V::ImportDefinition, V>(ctx, root, entry.1);
                    // }
                    // "data_types" => {
                    //     parse_collection::<V::DataTypeDefinition, V>(ctx, root, entry.1);
                    // }
                    // "node_types" => {
                    //     parse_collection::<V::NodeTypeDefinition, V>(ctx, root, entry.1);
                    // }
                    // "service_template" => {
                    //     let t = V::ServiceTemplateDefinition::parse::<V>(ctx, entry.1);
                    //     ctx.graph.add_edge(root, t, Relation::ServiceTemplate);
                    // }
                    f => {
                        let e = ast.add_node(ParseError::UnknownField(f.to_string()).into());
                        ast.add_edge(e, *k, ParseLoc.into());
                    }
                });

            root
        } else {
            let e = ast.add_node(ParseError::UnexpectedType("map").into());
            ast.add_edge(e, self.0, ParseLoc.into());

            self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::{dot::Dot, visit::EdgeRef, Directed};
    use toto_ast::Parse;

    use crate::{
        parse::ParseError,
        tosca::{AsYamlEntity, Test, TestRel, ToscaGrammar},
    };

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../../tests/tosca_2_0.yaml");

        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = petgraph::Graph::<Test, TestRel, Directed, u32>::new();

        let root = toto_yaml::Entity::from(yaml.clone()).parse(&mut ast);
        ToscaGrammar(root).parse(&mut ast);

        dbg!(Dot::new(&ast));

        let errors = ast
            .node_indices()
            .into_iter()
            .filter_map(|node| match &ast[node] {
                Test::Error(err) => Some((node, err)),
                _ => None,
            })
            .map(|(node, err)| {
                let pos: usize = ast
                    .edges(node)
                    .find_map(|e| match e.weight() {
                        TestRel::Error(_) => Some(e.target()),
                        _ => None,
                    })
                    .map(|n| ast[n].as_yaml().unwrap().0.pos())
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or_default();
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
