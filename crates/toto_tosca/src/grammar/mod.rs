use toto_parser::EntityParser;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub mod parser;
pub mod v1_3;
pub mod v2_0;

pub trait ToscaDefinitionsVersion {
    type Entity: ToscaCompatibleEntity;
    type Relation: ToscaCompatibleRelation;

    type ImportDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type FileDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    pub enum Entity {
        File(toto_yaml::FileEntity),
        Parse(toto_parser::ParseError),
        Yaml(toto_yaml::Entity),
        Tosca(crate::Entity),
    }

    #[derive(Debug)]
    pub enum Relation {
        File(toto_yaml::FileRelation),
        Parse(toto_parser::ParseLoc),
        Yaml(toto_yaml::Relation),
        Tosca(crate::Relation),
    }

    impl toto_yaml::AsYamlRelation for Relation {
        fn as_yaml(&self) -> Option<&toto_yaml::Relation> {
            match self {
                Relation::Yaml(value) => Some(value),
                _ => None,
            }
        }
    }

    impl toto_parser::AsParseLoc for Relation {
        fn as_parse_loc(&self) -> Option<&toto_parser::ParseLoc> {
            match self {
                Relation::Parse(value) => Some(value),
                _ => None,
            }
        }
    }

    impl toto_yaml::AsFileRelation for Relation {
        fn as_file(&self) -> Option<&toto_yaml::FileRelation> {
            match self {
                Relation::File(value) => Some(value),
                _ => None,
            }
        }
    }

    impl From<toto_parser::ParseError> for Entity {
        fn from(value: toto_parser::ParseError) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Entity> for Entity {
        fn from(value: toto_yaml::Entity) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<crate::Entity> for Entity {
        fn from(value: crate::Entity) -> Self {
            Self::Tosca(value)
        }
    }

    impl From<toto_parser::ParseLoc> for Relation {
        fn from(value: toto_parser::ParseLoc) -> Self {
            Self::Parse(value)
        }
    }

    impl From<toto_yaml::Relation> for Relation {
        fn from(value: toto_yaml::Relation) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<crate::Relation> for Relation {
        fn from(value: crate::Relation) -> Self {
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

    impl toto_parser::AsParseError for Entity {
        fn as_parse(&self) -> Option<&toto_parser::ParseError> {
            match self {
                Entity::Parse(value) => Some(value),
                _ => None,
            }
        }
    }

    impl toto_yaml::AsFileEntity for Entity {
        fn as_file(&self) -> Option<&toto_yaml::FileEntity> {
            match self {
                Entity::File(value) => Some(value),
                _ => None,
            }
        }
    }
}
