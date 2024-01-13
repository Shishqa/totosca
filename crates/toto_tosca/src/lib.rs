pub type Integer = i64;
pub type Float = ordered_float::OrderedFloat<f64>;
pub type Boolean = bool;
pub type Bytes = Vec<u8>;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Scalar {
    Integer(Integer),
    Float(Float),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RangeBound {
    Scalar(Scalar),
    Unbounded,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub minor: u64,
    pub major: u64,
    pub fix: u64,
    pub qualifier: String,
    pub build: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Range {
    pub lower: RangeBound,
    pub upper: RangeBound,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScalarUnit<U> {
    pub scalar: Scalar,
    pub unit: U,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Entity {
    File,
    ServiceTemplate,
    Repository,
    Import,
    DataType,
    Ref(String),
    Metadata(String),
    Schema,
    Node,
    NodeType,
    Attribute,
    Property,
    Parameter,
    Requirement,
    String(String),
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
    Nil,
    List,
    Map,
    Function,
    FunctionCall,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Relation {
    Type,
    DerivedFrom,
    Validation,
    Version,
    MapKey,
    MapValue,
    ListValue(u64),
    Description,
    KeySchema,
    EntrySchema,
    Subdef(String),
    Status,
    Default,
    Required,
    Value,
    Mapping,
    ExternalSchema,
    Count,
    CountRange,
    Node,
    Url,
    Namespace,
    Profile,
    Repository,
    ServiceTemplate,
    Function,
}
