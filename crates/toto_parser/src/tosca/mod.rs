use std::fmt::Debug;

use anyhow::anyhow;
use petgraph::{
    data::{Build, Create, DataMap, DataMapMut},
    visit::{Data, EdgeRef, GraphBase, IntoEdges},
};
use phf::phf_map;

use crate::parse::{ParseError, ParseLoc};

// use yaml_peg::node;
//
// use crate::parse::ParseError;
//
use self::{
    ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
    v2_0::{Tosca2_0, ToscaFileDefinition},
};

// pub mod v1_3;
pub mod ast;
pub mod v2_0;

pub trait ToscaDefinitionsVersion<E, R> {
    //type AttributeDefinition: toto_ast::Parse<E, R>;
    //type AttributeAssignment: toto_ast::Parse<E, R>;
    //type PropertyDefinition: toto_ast::Parse<E, R>;
    //type PropertyAssignment: toto_ast::Parse<E, R>;
    //type ParameterDefinition: toto_ast::Parse<E, R>;
    //type DataTypeDefinition: toto_ast::Parse<E, R>;
    //type NodeTypeDefinition: toto_ast::Parse<E, R>;
    //type NodeTemplateDefinition: toto_ast::Parse<E, R>;
    //type RequirementDefinition: toto_ast::Parse<E, R>;
    //type RequirementAssignment: toto_ast::Parse<E, R>;
    //type SchemaDefinition: toto_ast::Parse<E, R>;
    //type ImportDefinition: toto_ast::Parse<A>;
    //type ServiceTemplateDefinition: toto_ast::Parse<E, R>;
    type FileDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    //type Value: toto_ast::Parse<E, R>;
}

pub struct ToscaGrammar(toto_ast::GraphHandle);

impl<E, R> toto_ast::Parse<E, R> for ToscaGrammar
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let t = ast.node_weight(self.0).unwrap();
        let t = t.as_yaml().unwrap();

        match t.0.yaml() {
            yaml_peg::Yaml::Map(_) => {
                let version = ast
                    .edges(self.0)
                    .filter(|e| {
                        matches!(e.weight().as_yaml().unwrap(), &toto_yaml::Relation::MapKey)
                    })
                    .map(|e| e.target())
                    .find(|n| {
                        let t: &toto_yaml::Entity = ast.node_weight(*n).unwrap().as_yaml().unwrap();
                        matches!(t.0.yaml(), yaml_peg::Yaml::Str(key) if key == "tosca_definitions_version")
                    })
                    .map(|n| {
                        ast.edges(n).find(|e| matches!(e.weight().as_yaml().unwrap(), &toto_yaml::Relation::MapValue)).unwrap()
                    }).map(|e| e.target());

                match version {
                    Some(version_node) => {
                        let t: &toto_yaml::Entity =
                            ast.node_weight(version_node).unwrap().as_yaml().unwrap();
                        match t.0.yaml() {
                            yaml_peg::Yaml::Str(version_str) => match version_str.as_str() {
                                "tosca_2_0" => <<Tosca2_0 as ToscaDefinitionsVersion<E, R>>::FileDefinition as From<toto_ast::GraphHandle>>::from(self.0).parse(ast),
                                _ => self.0,
                            },
                            _ => self.0,
                        }
                    }
                    None => {
                        let e = ast
                            .add_node(ParseError::MissingField("tosca_definitions_version").into());
                        ast.add_edge(e, self.0, ParseLoc.into());
                        self.0
                    }
                }
            }
            _ => {
                let e = ast.add_node(ParseError::UnexpectedType("map").into());
                ast.add_edge(e, self.0, ParseLoc.into());
                self.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use toto_ast::Parse;
    use toto_yaml::Entity;

    use crate::parse::{ParseError, ParseLoc};

    use super::ToscaGrammar;

    #[derive(Debug)]
    pub enum Test {
        Parse(ParseError),
        Yaml(toto_yaml::Entity),
        Tosca(toto_tosca::Entity),
    }

    #[derive(Debug)]
    pub enum TestRel {
        Parse(ParseLoc),
        Yaml(toto_yaml::Relation),
        Tosca(toto_tosca::Relation),
    }

    impl toto_yaml::AsYamlRelation for TestRel {
        fn as_yaml(&self) -> Option<&toto_yaml::Relation> {
            match self {
                TestRel::Yaml(value) => Some(value),
                _ => None,
            }
        }
    }

    impl From<ParseError> for Test {
        fn from(value: ParseError) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Entity> for Test {
        fn from(value: toto_yaml::Entity) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<toto_tosca::Entity> for Test {
        fn from(value: toto_tosca::Entity) -> Self {
            Self::Tosca(value)
        }
    }

    impl From<ParseLoc> for TestRel {
        fn from(value: ParseLoc) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Relation> for TestRel {
        fn from(value: toto_yaml::Relation) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<toto_tosca::Relation> for TestRel {
        fn from(value: toto_tosca::Relation) -> Self {
            Self::Tosca(value)
        }
    }

    impl toto_yaml::AsYamlEntity for Test {
        fn as_yaml(&self) -> Option<&toto_yaml::Entity> {
            match self {
                Test::Yaml(value) => Some(value),
                _ => None,
            }
        }
    }

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../tests/a.yaml");

        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = toto_ast::AST::<Test, TestRel>::new();

        let root = Entity::from(yaml.clone()).parse(&mut ast);
        ToscaGrammar(root).parse(&mut ast);

        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
