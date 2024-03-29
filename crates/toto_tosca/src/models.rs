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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Entity {
    Import,
    File,
    Profile,
    ServiceTemplate,
    Repository,
    Node,
    Data,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Relation {
    ToscaDefinitionsVersion,
    ServiceTemplate,
    Repository,
    Profile,
    Description,
    Url,
    Namespace,
    Import(usize),

    ImportTarget,
    ImportFile,
    ImportProfile,
    ImportRepository,
    ImportNamespace,

    Type(String),
    Definition(String),
    Assignment(String),

    Metadata(String),
    Schema,
    Attribute(String),
    Property(String),
    Parameter(String),
    Requirement(usize, String),

    Function,
    FunctionCall,
    Value,

    RefRoot,
    RefSelf,

    HasType,
    DerivedFrom,
    RefType,
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
