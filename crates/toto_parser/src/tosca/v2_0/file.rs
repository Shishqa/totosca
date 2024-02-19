use std::marker::PhantomData;

use crate::{
    parse::{add_error, ParseError, ParseLoc, StaticSchema},
    tosca::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion},
};
use toto_ast::Parse;

use super::{Collection, List};

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

impl<E, R, V> StaticSchema<E, R> for ToscaFileDefinition<E, R, V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<E, R>,
{
    const ROOT: toto_tosca::Entity = toto_tosca::Entity::File;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    > = phf::phf_map! {
        "tosca_definitions_version" => |r, n, ast| {
            let t =
                ast.add_node(toto_tosca::Entity::ToscaDefinitionsVersion.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "profile" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Profile.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "metadata" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Metadata.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "description" => |r, n, ast| {
            let t = ast.add_node(toto_tosca::Entity::Description.into());
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
            ast.add_edge(t, n, ParseLoc.into());
        },
        "imports" => |r, n, ast| {
            List::<E, R, V::ImportDefinition>(n, r, PhantomData::default())
                .parse(ast);
        },
        "data_types" => |r, n, ast| {
            Collection::<E, R, V::DataTypeDefinition>(
                n,
                r,
                PhantomData::default(),
            )
            .parse(ast);
        },
        "node_types" => |r, n, ast| {
            Collection::<E, R, V::NodeTypeDefinition>(
                n,
                r,
                PhantomData::default(),
            )
            .parse(ast);
        },
        "service_template" => |r, n, ast| {
            let t = V::ServiceTemplateDefinition::from(n).parse(ast);
            ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        }
    };
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

        if let toto_yaml::Entity::Map = t {
            Self::parse_schema(self.0, ast)
        } else {
            add_error(self.0, ast, ParseError::UnexpectedType("map"));
            self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use ariadne::{Label, Report, ReportKind, Source};
    use petgraph::{
        dot::Dot,
        visit::{EdgeFiltered, EdgeRef, NodeFiltered},
    };
    use toto_ast::Parse;

    use crate::{
        parse::ParseError,
        tosca::{tests::Entity, tests::Relation, ToscaGrammar},
    };

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../../tests/tosca_2_0.yaml");
        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = toto_ast::AST::<Entity, Relation>::new();
        let doc_handle = toto_yaml::FileEntity(doc.to_string()).parse(&mut ast);

        let root = toto_yaml::Yaml(yaml.clone(), doc_handle).parse(&mut ast);
        ToscaGrammar(root).parse(&mut ast);

        let ast_filtered = NodeFiltered::from_fn(&ast, |n| matches!(ast[n], Entity::Tosca(_)));

        dbg!(Dot::new(&ast_filtered));

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
