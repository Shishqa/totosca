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
    ImportUrl,
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
