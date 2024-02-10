// pub mod attribute;
//pub mod collection;
// pub mod data;
pub mod file;
//pub mod import;
// pub mod list;
// pub mod map;
//pub mod node;
// pub mod parameter;
// pub mod property;
// pub mod reference;
// pub mod requirement;
// pub mod schema;
//pub mod service_template;
//pub mod value;

// pub use attribute::*;
//pub use collection::*;
// pub use data::*;
pub use file::*;
//pub use import::*;
// pub use list::*;
// pub use map::*;
//pub use node::*;
// pub use parameter::*;
// pub use property::*;
// pub use reference::*;
// pub use requirement::*;
// pub use schema::*;
//pub use service_template::*;
//pub use value::*;

use super::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion};

pub struct Tosca2_0;

impl<E, R> ToscaDefinitionsVersion<E, R> for Tosca2_0
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    //type AttributeDefinition = AttributeDefinition;
    //type AttributeAssignment = AttributeAssignment;
    //type PropertyAssignment = PropertyAssignment;
    //type PropertyDefinition = PropertyDefinition;
    //type ParameterDefinition = ParameterDefinition;
    type FileDefinition = ToscaFileDefinition<E, R, Self>;
    //type ImportDefinition = ImportDefinition<A, Self>;
    //type SchemaDefinition = SchemaDefinition;
    //type RequirementDefinition = RequirementDefinition;
    //type RequirementAssignment = RequirementAssignment;
    //type DataTypeDefinition = DataType;
    //type NodeTypeDefinition = NodeType;
    //type NodeTemplateDefinition = NodeTemplate;
    //type ServiceTemplateDefinition = ServiceTemplateDefinition;
    //type Value = Value<Self>;
}
