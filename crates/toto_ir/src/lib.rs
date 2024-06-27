extern crate derive_more;
use derive_more::{From, TryInto};

#[derive(Debug, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Entity {
    File(toto_yaml::FileEntity),
    Parse(toto_parser::ParseError),
    Yaml(toto_yaml::Entity),
    Tosca(toto_tosca::Entity),
}

#[derive(Debug, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Relation {
    File(toto_yaml::FileRelation),
    Parse(toto_parser::ParseLoc),
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

impl toto_tosca::AsToscaRelation for Relation {
    fn as_tosca(&self) -> Option<&toto_tosca::Relation> {
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

impl toto_tosca::AsToscaEntity for Entity {
    fn as_tosca(&self) -> Option<&toto_tosca::Entity> {
        match self {
            Entity::Tosca(value) => Some(value),
            _ => None,
        }
    }
}
