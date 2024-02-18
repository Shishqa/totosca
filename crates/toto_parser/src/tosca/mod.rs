use std::fmt::Debug;

use anyhow::anyhow;
use petgraph::{
    data::{Build, Create, DataMap, DataMapMut},
    visit::{Data, EdgeRef, GraphBase, IntoEdges},
};
use phf::phf_map;

use crate::parse::{add_error, ParseError, ParseLoc};

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
    type AttributeDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type AttributeAssignment: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type PropertyDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type PropertyAssignment: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type ParameterDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type DataTypeDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type NodeTypeDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type NodeTemplateDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type RequirementDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type RequirementAssignment: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type SchemaDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type ImportDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type ServiceTemplateDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type FileDefinition: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
    type Value: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
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

        match t {
            toto_yaml::Entity::Map => {
                let version = toto_yaml::iter_keys(self.0, ast).find_map(|(k, v)| {
                    match ast.node_weight(k).unwrap().as_yaml() {
                        Some(toto_yaml::Entity::Str(key)) if key == "tosca_definitions_version" => {
                            Some(v)
                        }
                        _ => None,
                    }
                });

                match version {
                    Some(version_node) => {
                        let t: &toto_yaml::Entity =
                            ast.node_weight(version_node).unwrap().as_yaml().unwrap();
                        match t {
                            toto_yaml::Entity::Str(version_str) => match version_str.as_str() {
                                "tosca_2_0" => <<Tosca2_0 as ToscaDefinitionsVersion<E, R>>::FileDefinition as From<toto_ast::GraphHandle>>::from(self.0).parse(ast),
                                _ => self.0,
                            },
                            _ => self.0,
                        }
                    }
                    None => {
                        add_error(
                            self.0,
                            ast,
                            ParseError::MissingField("tosca_definitions_version"),
                        );
                        self.0
                    }
                }
            }
            _ => {
                add_error(self.0, ast, ParseError::UnexpectedType("map"));
                self.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use toto_ast::Parse;

    use crate::parse::{ParseError, ParseLoc};

    use super::ToscaGrammar;

    #[derive(Debug)]
    pub enum Entity {
        File(toto_yaml::FileEntity),
        Parse(ParseError),
        Yaml(toto_yaml::Entity),
        Tosca(toto_tosca::Entity),
    }

    #[derive(Debug)]
    pub enum Relation {
        File(toto_yaml::FileRelation),
        Parse(ParseLoc),
        Yaml(toto_yaml::Relation),
        Tosca(toto_tosca::Relation),
    }

    impl toto_yaml::AsYamlRelation for Relation {
        fn as_yaml(&self) -> Option<&toto_yaml::Relation> {
            match self {
                Relation::Yaml(value) => Some(value),
                _ => None,
            }
        }
    }

    impl From<ParseError> for Entity {
        fn from(value: ParseError) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Entity> for Entity {
        fn from(value: toto_yaml::Entity) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<toto_tosca::Entity> for Entity {
        fn from(value: toto_tosca::Entity) -> Self {
            Self::Tosca(value)
        }
    }

    impl From<ParseLoc> for Relation {
        fn from(value: ParseLoc) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Relation> for Relation {
        fn from(value: toto_yaml::Relation) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<toto_tosca::Relation> for Relation {
        fn from(value: toto_tosca::Relation) -> Self {
            Self::Tosca(value)
        }
    }

    impl From<toto_yaml::FileRelation> for Relation {
        fn from(value: toto_yaml::FileRelation) -> Self {
            Self::File(value)
        }
    }

    impl From<toto_yaml::FileEntity> for Entity {
        fn from(value: toto_yaml::FileEntity) -> Self {
            Self::File(value)
        }
    }

    impl toto_yaml::AsYamlEntity for Entity {
        fn as_yaml(&self) -> Option<&toto_yaml::Entity> {
            match self {
                Entity::Yaml(value) => Some(value),
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

        let mut ast = toto_ast::AST::<Entity, Relation>::new();
        let doc_handle = toto_yaml::FileEntity(doc.to_string()).parse(&mut ast);

        let root = toto_yaml::Yaml(yaml, doc_handle).parse(&mut ast);
        ToscaGrammar(root).parse(&mut ast);

        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
