use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub mod collection;
pub mod field;
pub mod hierarchy;
pub mod list;
pub mod parser;
// pub mod v1_3;
pub mod v2_0;

pub trait ToscaDefinitionsVersion {
    type Entity: ToscaCompatibleEntity;
    type Relation: ToscaCompatibleRelation;

    type ImportDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type FileDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type ServiceTemplateDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type NodeTypeDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type NodeTemplateDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type DataTypeDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type SchemaDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type PropertyDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type AttributeDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type ParameterDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type ArtifactTypeDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
    type ArtifactDefinition: toto_parser::EntityParser<Self::Entity, Self::Relation>;
}

#[cfg(test)]
mod tests {
    extern crate derive_more;
    use derive_more::{From, TryInto};

    #[derive(Debug, From, TryInto)]
    #[try_into(owned, ref, ref_mut)]
    pub enum Entity {
        File(toto_yaml::FileEntity),
        Parse(toto_parser::ParseError),
        Yaml(toto_yaml::Entity),
        Tosca(crate::Entity),
    }

    #[derive(Debug, From, TryInto)]
    #[try_into(owned, ref, ref_mut)]
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

    impl crate::AsToscaRelation for Relation {
        fn as_tosca(&self) -> Option<&crate::Relation> {
            match self {
                Relation::Tosca(value) => Some(value),
                _ => None,
            }
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

    impl crate::AsToscaEntity for Entity {
        fn as_tosca(&self) -> Option<&crate::Entity> {
            match self {
                Entity::Tosca(value) => Some(value),
                _ => None,
            }
        }
    }
}
