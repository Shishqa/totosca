use crate::parse::{Context, GraphHandle};

use super::{v2_0, Parse, ToscaDefinitionsVersion};

pub struct Tosca1_3;

pub mod attribute;
pub mod file;
pub mod value;

impl ToscaDefinitionsVersion for Tosca1_3 {
    type AttributeDefinition = v2_0::AttributeDefinition;
    type AttributeAssignment = attribute::AttributeAssignment;
    type PropertyAssignment = v2_0::PropertyAssignment;
    type PropertyDefinition = v2_0::PropertyDefinition;
    type ParameterDefinition = v2_0::ParameterDefinition;
    type FileDefinition = file::ToscaFileDefinition;
    type ImportDefinition = v2_0::ImportDefinition;
    type SchemaDefinition = v2_0::SchemaDefinition;
    type RequirementDefinition = v2_0::RequirementDefinition;
    type RequirementAssignment = v2_0::RequirementAssignment;
    type DataTypeDefinition = v2_0::DataType;
    type NodeTypeDefinition = v2_0::NodeType;
    type NodeTemplateDefinition = v2_0::NodeTemplate;
    type ServiceTemplateDefinition = v2_0::ServiceTemplateDefinition;
    type Value = value::Value;

    fn parse(ctx: &mut Context, n: &yaml_peg::NodeRc) -> GraphHandle {
        return Self::FileDefinition::parse::<Self>(ctx, n);
    }
}
