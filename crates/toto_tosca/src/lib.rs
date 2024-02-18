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
    ToscaDefinitionsVersion,
    File,
    ServiceTemplate,
    Repository,
    Import,
    Profile,
    Description,
    DataType,
    Ref { name: String },
    Metadata,
    Schema,
    Node,
    NodeType,
    Attribute,
    Property,
    Parameter,
    Requirement,
    Function,
    FunctionCall,
    Url,
    Namespace,
    Value,
}

#[derive(Debug)]
pub enum Relation {
    Subdef,
    OrderedSubdef(usize),
    NamedSubdef(String),
    HasType,
    DerivedFrom,
}
