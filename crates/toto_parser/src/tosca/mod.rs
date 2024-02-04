use std::fmt::Debug;

use anyhow::anyhow;
use petgraph::{data::Build, visit::EdgeRef};

use crate::parse::{ParseError, ParseErrorLoc};

// use yaml_peg::node;
//
// use crate::parse::ParseError;
//
// use self::{v1_3::Tosca1_3, v2_0::Tosca2_0};

// pub mod v1_3;
// pub mod v2_0;

pub trait ToscaDefinitionsVersion<E, R> {
    type AttributeDefinition: toto_ast::TransformAST<E, R>;
    type AttributeAssignment: toto_ast::TransformAST<E, R>;
    type PropertyDefinition: toto_ast::TransformAST<E, R>;
    type PropertyAssignment: toto_ast::TransformAST<E, R>;
    type ParameterDefinition: toto_ast::TransformAST<E, R>;
    type DataTypeDefinition: toto_ast::TransformAST<E, R>;
    type NodeTypeDefinition: toto_ast::TransformAST<E, R>;
    type NodeTemplateDefinition: toto_ast::TransformAST<E, R>;
    type RequirementDefinition: toto_ast::TransformAST<E, R>;
    type RequirementAssignment: toto_ast::TransformAST<E, R>;
    type SchemaDefinition: toto_ast::TransformAST<E, R>;
    type ImportDefinition: toto_ast::TransformAST<E, R>;
    type ServiceTemplateDefinition: toto_ast::TransformAST<E, R>;
    type FileDefinition: toto_ast::TransformAST<E, R>;
    type Value: toto_ast::TransformAST<E, R>;
}

pub struct ToscaGrammar;

#[derive(Debug)]
pub enum Test {
    Error(ParseError),
    Yaml(toto_yaml::Entity),
}

#[derive(Debug)]
pub enum TestRel {
    Error(ParseErrorLoc),
    Yaml(toto_yaml::Relation),
}

impl<'a> TryFrom<&'a Test> for &'a toto_yaml::Entity {
    type Error = String;

    fn try_from(value: &'a Test) -> Result<Self, Self::Error> {
        match value {
            Test::Yaml(value) => Ok(value),
            _ => Err("nothing".to_string()),
        }
    }
}

impl From<ParseError> for Test {
    fn from(value: ParseError) -> Self {
        Self::Error(value)
    }
}

impl From<toto_yaml::Entity> for Test {
    fn from(value: toto_yaml::Entity) -> Self {
        Self::Yaml(value)
    }
}

impl From<ParseErrorLoc> for TestRel {
    fn from(value: ParseErrorLoc) -> Self {
        Self::Error(value)
    }
}

impl From<toto_yaml::Relation> for TestRel {
    fn from(value: toto_yaml::Relation) -> Self {
        Self::Yaml(value)
    }
}

impl<'a> TryFrom<&'a TestRel> for &'a toto_yaml::Relation {
    type Error = String;

    fn try_from(value: &'a TestRel) -> Result<Self, Self::Error> {
        match value {
            TestRel::Yaml(value) => Ok(value),
            _ => Err("nothing".to_string()),
        }
    }
}

impl<E, R, D> toto_ast::TransformAST<E, R> for ToscaGrammar
where
    D: Debug,
    for<'a> &'a toto_yaml::Entity: TryFrom<&'a E, Error = D>,
    for<'a> &'a toto_yaml::Relation: TryFrom<&'a R, Error = D>,
    E: From<ParseError>,
    R: From<ParseErrorLoc>,
{
    fn transform_ast(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle {
        let t = &ast[n];

        let t: &toto_yaml::Entity = t.try_into().unwrap();

        match t.0.yaml() {
            yaml_peg::Yaml::Map(_) => {
                let version = ast
                    .edges(n)
                    .filter(|e| {
                        matches!(e.weight().try_into().unwrap(), &toto_yaml::Relation::MapKey)
                    })
                    .map(|e| e.target())
                    .find(|n| {
                        let t: &toto_yaml::Entity = (&ast[*n]).try_into().unwrap();
                        matches!(t.0.yaml(), yaml_peg::Yaml::Str(key) if key == "tosca_definitions_version")
                    })
                    .map(|n| {
                        ast.edges(n).find(|e| matches!(e.weight().try_into().unwrap(), &toto_yaml::Relation::MapValue)).unwrap()
                    }).map(|e| e.target());

                match version {
                    Some(version_node) => {
                        let t: &toto_yaml::Entity = (&ast[version_node]).try_into().unwrap();
                        match t.0.yaml() {
                            yaml_peg::Yaml::Str(version_str) => match version_str.as_str() {
                                "tosca_simple_yaml_1_3" => n,
                                "tosca_2_0" => n,
                                _ => n,
                            },
                            _ => n,
                        }
                    }
                    None => {
                        let e = ast
                            .add_node(ParseError::MissingField("tosca_definitions_version").into());
                        ast.add_edge(e, n, ParseErrorLoc.into());
                        n
                    }
                }
            }
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map").into());
                ast.add_edge(e, n, ParseErrorLoc.into());
                n
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tosca::Test;

    use petgraph::dot::Dot;
    use toto_ast::{ToAST, TransformAST};
    use toto_yaml::Entity;

    use super::{TestRel, ToscaGrammar};

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../tests/a.yaml");

        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = toto_ast::AST::<Test, TestRel>::new();

        let root = Entity::from(yaml.clone()).to_ast(&mut ast);

        ToscaGrammar::transform_ast(root, &mut ast);

        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
