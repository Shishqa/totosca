#[derive(Debug)]
pub enum Entity {
    File(toto_yaml::FileEntity),
    Parse(toto_parser::ParseError),
    Yaml(toto_yaml::Entity),
    Tosca(toto_tosca::Entity),
}

#[derive(Debug)]
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

impl From<toto_tosca::Entity> for Entity {
    fn from(value: toto_tosca::Entity) -> Self {
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
