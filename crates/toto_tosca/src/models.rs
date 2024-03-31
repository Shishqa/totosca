extern crate derive_more;
use derive_more::{From, TryInto};

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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ImportEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FileEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ServiceTemplateEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RepositoryEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DataEntity;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Entity {
    Import(ImportEntity),
    File(FileEntity),
    ServiceTemplate(ServiceTemplateEntity),
    Repository(RepositoryEntity),
    Node(NodeEntity),
    Data(DataEntity),
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ToscaDefinitionsVersionRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ServiceTemplateRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RepositoryRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ProfileRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct DescriptionRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct NamespaceRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct VersionRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportRelation(pub usize);

impl From<usize> for ImportRelation {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportUrlRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportTargetRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportFileRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportProfileRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportRepositoryRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ImportNamespaceRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct TypeRelation(pub String);

impl From<String> for TypeRelation {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct DefinitionRelation(pub String);

impl From<String> for DefinitionRelation {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct AssignmentRelation(pub String);

impl From<String> for AssignmentRelation {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct MetadataRelation(pub String);

impl From<String> for MetadataRelation {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct SchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct KeySchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EntrySchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RefRootRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct HasTypeRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RefHasTypeRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct DerivedFromRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RefDerivedFromRelation;

#[derive(Debug, PartialEq, Eq, Hash, Clone, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Relation {
    ToscaDefinitionsVersion(ToscaDefinitionsVersionRelation),
    ServiceTemplate(ServiceTemplateRelation),
    Repository(RepositoryRelation),
    Profile(ProfileRelation),
    Description(DescriptionRelation),
    Namespace(NamespaceRelation),
    Version(VersionRelation),
    Import(ImportRelation),

    ImportTarget(ImportTargetRelation),
    ImportUrl(ImportUrlRelation),
    ImportFile(ImportFileRelation),
    ImportProfile(ImportProfileRelation),
    ImportRepository(ImportRepositoryRelation),
    ImportNamespace(ImportNamespaceRelation),

    Type(TypeRelation),
    Definition(DefinitionRelation),
    Assignment(AssignmentRelation),

    Metadata(MetadataRelation),

    Schema(SchemaRelation),
    KeySchema(KeySchemaRelation),
    EntrySchema(EntrySchemaRelation),

    RefRoot(RefRootRelation),

    HasType(HasTypeRelation),
    RefHasType(RefHasTypeRelation),

    DerivedFrom(DerivedFromRelation),
    RefDerivedFrom(RefDerivedFromRelation),
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
