pub mod attribute;
pub mod collection;
pub mod data;
pub mod file;
pub mod import;
pub mod list;
pub mod node;
pub mod parameter;
pub mod property;
pub mod requirement;
pub mod schema;
pub mod service_template;
pub mod value;

pub use attribute::*;
pub use collection::*;
pub use data::*;
pub use file::*;
pub use import::*;
pub use list::*;
pub use node::*;
pub use parameter::*;
pub use property::*;
pub use requirement::*;
pub use schema::*;
pub use service_template::*;
pub use value::*;

use super::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion};

pub struct Tosca2_0;

impl<E, R> ToscaDefinitionsVersion<E, R> for Tosca2_0
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    type AttributeDefinition = self::AttributeDefinition<E, R, Self>;
    type AttributeAssignment = self::AttributeAssignment<E, R, Self>;
    type PropertyAssignment = self::PropertyAssignment<E, R, Self>;
    type PropertyDefinition = self::PropertyDefinition<E, R, Self>;
    type ParameterDefinition = self::ParameterDefinition<E, R, Self>;
    type FileDefinition = self::ToscaFileDefinition<E, R, Self>;
    type ImportDefinition = self::ImportDefinition<E, R, Self>;
    type SchemaDefinition = self::SchemaDefinition<E, R, Self>;
    type RequirementDefinition = self::RequirementDefinition<E, R, Self>;
    type RequirementAssignment = self::RequirementAssignment<E, R, Self>;
    type DataTypeDefinition = self::DataType<E, R, Self>;
    type NodeTypeDefinition = self::NodeType<E, R, Self>;
    type NodeTemplateDefinition = self::NodeTemplate<E, R, Self>;
    type ServiceTemplateDefinition = self::ServiceTemplateDefinition<E, R, Self>;
    type Value = self::Value<E, R, Self>;
}
