extern crate derive_more;

use derive_more::{From, TryInto};

use crate::semantic::SimpleLookuper;

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
pub struct SubstitutionMappingEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RepositoryEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DataEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ArtifactEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CapabilityEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct InterfaceEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct OperationEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NotificationEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RelationshipEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RequirementEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GroupEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PolicyEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PolicyTriggerEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ImplementationEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowStepEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowDelegateActivityEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowInlineActivityEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowSetStateActivityEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WorkflowCallOperationActivityEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FunctionEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FunctionSignatureEntity;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StatusEntity {
    #[default]
    Supported,
    Unsupported,
    Experimental,
    Deprecated,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Entity {
    Import(ImportEntity),
    File(FileEntity),
    ServiceTemplate(ServiceTemplateEntity),
    SubstitutionMapping(SubstitutionMappingEntity),
    Repository(RepositoryEntity),
    Node(NodeEntity),
    Data(DataEntity),
    Status(StatusEntity),
    Artifact(ArtifactEntity),
    Capability(CapabilityEntity),
    Interface(InterfaceEntity),
    Operation(OperationEntity),
    Notification(NotificationEntity),
    Relationship(RelationshipEntity),
    Requirement(RequirementEntity),
    Group(GroupEntity),
    Policy(PolicyEntity),
    PolicyTrigger(PolicyTriggerEntity),
    Implementation(ImplementationEntity),

    Workflow(WorkflowEntity),
    WorkflowStep(WorkflowStepEntity),
    WorkflowDelegateActivity(WorkflowDelegateActivityEntity),
    WorkflowInlineActivity(WorkflowInlineActivityEntity),
    WorkflowSetStateActivity(WorkflowSetStateActivityEntity),
    WorkflowCallOperationActivity(WorkflowCallOperationActivityEntity),

    Function(FunctionEntity),
    FunctionSignature(FunctionSignatureEntity),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RefRelation {
    pub lookuper: Box<SimpleLookuper>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ToscaDefinitionsVersionRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ServiceTemplateRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct SubstitutionMappingRelation;

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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct ImportRelation(pub usize);

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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct TypeRelation(pub String);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct DefinitionRelation(pub String);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct OrderedDefinitionRelation(pub (String, usize));

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct AssignmentRelation(pub String);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct OrderedAssignmentRelation(pub (String, usize));

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct MetadataRelation(pub String);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct SchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RequiredRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ValidationRelation;

impl From<usize> for ValidationRelation {
    fn from(_: usize) -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ValueRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct MappingRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct DefaultRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct StatusRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ExternalSchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct KeySchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EntrySchemaRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct RootRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct HasTypeRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct SubstitutesTypeRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct DerivedFromRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct MimeTypeRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct FileExtRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct HasFileRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ChecksumRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ChecksumAlgorithmRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PrimaryArtifactRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct DependencyArtifactRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct ValidSourceNodeTypeRelation;

impl From<usize> for ValidSourceNodeTypeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct MemberNodeTypeRelation;

impl From<usize> for MemberNodeTypeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct MemberNodeTemplateRelation;

impl From<usize> for MemberNodeTemplateRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct ValidRelationshipTypeRelation;

impl From<usize> for ValidRelationshipTypeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct DirectiveRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct ValidCapabilityTypeRelation;

impl From<usize> for ValidCapabilityTypeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct ValidTargetNodeTypeRelation;

impl From<usize> for ValidTargetNodeTypeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct TargetNodeRelation;

impl From<usize> for TargetNodeRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct TargetCapabilityRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct WorkflowActivityRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct WorkflowRelation;

impl From<usize> for WorkflowRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct OperationRelation;

impl From<usize> for OperationRelation {
    fn from(_: usize) -> Self {
        Self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PolicyTriggerEventRelation;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct FunctionArgumentRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct FunctionOptionalArgumentRelation(pub usize);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, From)]
pub struct FunctionSignatureRelation(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Relation {
    Ref(RefRelation),

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
    OrderedDefinition(OrderedDefinitionRelation),
    Assignment(AssignmentRelation),
    OrderedAssignment(OrderedAssignmentRelation),

    Validation(ValidationRelation),
    Value(ValueRelation),
    Mapping(MappingRelation),
    Required(RequiredRelation),
    Status(StatusRelation),
    Default(DefaultRelation),
    ExternalSchema(ExternalSchemaRelation),

    Metadata(MetadataRelation),

    Schema(SchemaRelation),
    KeySchema(KeySchemaRelation),
    EntrySchema(EntrySchemaRelation),

    Root(RootRelation),

    HasType(HasTypeRelation),
    DerivedFrom(DerivedFromRelation),
    SubstitutesType(SubstitutesTypeRelation),

    MimeType(MimeTypeRelation),
    FileExt(FileExtRelation),
    HasFile(HasFileRelation),
    Checksum(ChecksumRelation),
    ChecksumAlgorithm(ChecksumAlgorithmRelation),

    PrimaryArtifact(PrimaryArtifactRelation),
    DependencyArtifact(DependencyArtifactRelation),

    ValidSourceNodeType(ValidSourceNodeTypeRelation),
    ValidRelationshipType(ValidRelationshipTypeRelation),

    Directive(DirectiveRelation),

    MemberNodeTemplate(MemberNodeTemplateRelation),
    MemberNodeType(MemberNodeTypeRelation),

    ValidCapabilityType(ValidCapabilityTypeRelation),
    ValidTargetNodeType(ValidTargetNodeTypeRelation),

    TargetNode(TargetNodeRelation),
    TargetCapability(TargetCapabilityRelation),

    SubstitutionMapping(SubstitutionMappingRelation),

    WorkflowActivity(WorkflowActivityRelation),
    Workflow(WorkflowRelation),
    Operation(OperationRelation),

    PolicyTriggerEvent(PolicyTriggerEventRelation),

    FunctionArgument(FunctionArgumentRelation),
    FunctionOptionalArgument(FunctionOptionalArgumentRelation),
    FunctionSignature(FunctionSignatureRelation),
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
