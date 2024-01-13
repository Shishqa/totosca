use toto_tosca::{Entity, Relation};

use crate::parse::{Context, GraphHandle};

use super::{v2_0, Parse, ToscaDefinitionsVersion};

pub struct Tosca1_3;

pub mod attribute;
pub mod file;
pub mod value;

impl ToscaDefinitionsVersion for Tosca1_3 {
    type AttributeDefinition = v2_0::AttributeDefinition;
    type AttributeAssignment = attribute::AttributeAssignment;
    type PropertyAssignment = value::Value;
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
        let root = ctx.graph.add_node(Entity::File);
        let builtin_url = ctx.graph.add_node(Entity::String("$builtin".to_string()));
        ctx.graph.add_edge(root, builtin_url, Relation::Url);

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("get_input".to_string()));

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("get_attribute".to_string()));

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("get_property".to_string()));

        return Self::FileDefinition::parse::<Self>(ctx, n);
    }
}
