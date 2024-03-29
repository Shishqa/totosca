#[derive(Debug)]
pub struct Version {
    pub minor: u64,
    pub major: u64,
    pub fix: u64,
    pub qualifier: String,
    pub build: String,
}

#[derive(Debug)]
pub enum UnitSize {
    B,
    KB,
    KiB,
    MB,
    MiB,
    GB,
    GiB,
    TB,
    TiB,
}

#[derive(Debug)]
pub enum Entity {
    File,
    Definition,
}

#[derive(Debug)]
pub enum Relation {
    ToscaDefinitionsVersion,
    ServiceTemplate,
    Repository,
    Profile,
    Description,
    Url,
    Namespace,
    Import(usize),
    ImportFile,
    ImportProfile,
    ImportRepository,
    ImportNamespace,

    DataType(String),
    Metadata(String),
    Schema,
    NodeTemplate(String),
    NodeType(String),
    Attribute(String),
    Property(String),
    Parameter(String),
    Requirement(usize, String),

    Function,
    FunctionCall,
    Value,

    HasType,
    DerivedFrom,
    Validation,
    KeySchema,
    EntrySchema,
}

pub trait AsToscaEntity {
    fn as_tosca(&self) -> Option<&Entity>;
}

pub trait AsToscaRelation {
    fn as_tosca(&self) -> Option<&Relation>;
}

pub trait ToscaCompatibleEntity:
    toto_parser::ParseCompatibleEntity
    + From<toto_yaml::FileEntity>
    + From<toto_yaml::Entity>
    + From<crate::Entity>
    + AsToscaEntity
    + Sized
    + 'static
{
}

impl<T> ToscaCompatibleEntity for T where
    T: toto_parser::ParseCompatibleEntity
        + From<toto_yaml::FileEntity>
        + From<toto_yaml::Entity>
        + From<crate::Entity>
        + AsToscaEntity
        + Sized
        + 'static
{
}

pub trait ToscaCompatibleRelation:
    toto_parser::ParseCompatibleRelation
    + From<toto_yaml::FileRelation>
    + From<toto_yaml::Relation>
    + From<crate::Relation>
    + AsToscaRelation
    + Sized
    + 'static
{
}

impl<T> ToscaCompatibleRelation for T where
    T: toto_parser::ParseCompatibleRelation
        + From<toto_yaml::FileRelation>
        + From<toto_yaml::Relation>
        + From<crate::Relation>
        + AsToscaRelation
        + Sized
        + 'static
{
}
