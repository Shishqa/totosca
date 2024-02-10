use petgraph::visit::EdgeRef;

use crate::parse::{ParseError, ParseLoc};

pub trait ToscaCompatibleEntity:
    toto_yaml::AsYamlEntity + From<ParseError> + From<toto_tosca::Entity>
{
}

impl<T> ToscaCompatibleEntity for T where
    T: toto_yaml::AsYamlEntity + From<ParseError> + From<toto_tosca::Entity>
{
}

pub trait ToscaCompatibleRelation
where
    Self: toto_yaml::AsYamlRelation,
    Self: From<ParseLoc>,
    Self: From<toto_tosca::Relation>,
{
}

impl<T> ToscaCompatibleRelation for T where
    T: toto_yaml::AsYamlRelation + From<ParseLoc> + From<toto_tosca::Relation>
{
}
