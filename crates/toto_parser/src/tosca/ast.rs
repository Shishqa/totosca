use crate::parse::{ParseError, ParseLoc};

pub trait ToscaCompatibleEntity:
    toto_yaml::AsYamlEntity + From<ParseError> + From<toto_tosca::Entity> + 'static
{
}

impl<T> ToscaCompatibleEntity for T where
    T: toto_yaml::AsYamlEntity + From<ParseError> + From<toto_tosca::Entity> + 'static
{
}

pub trait ToscaCompatibleRelation:
    toto_yaml::AsYamlRelation + From<ParseLoc> + From<toto_tosca::Relation> + 'static
{
}

impl<T> ToscaCompatibleRelation for T where
    T: toto_yaml::AsYamlRelation + From<ParseLoc> + From<toto_tosca::Relation> + 'static
{
}
