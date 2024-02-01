pub mod attribute;
pub mod collection;
pub mod data;
pub mod file;
pub mod import;
pub mod list;
pub mod map;
pub mod node;
pub mod parameter;
pub mod property;
pub mod reference;
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
pub use map::*;
pub use node::*;
pub use parameter::*;
pub use property::*;
pub use reference::*;
pub use requirement::*;
pub use schema::*;
pub use service_template::*;
use toto_tosca::{Entity, Relation};
pub use value::*;

use crate::tosca::Parse;

use super::ToscaDefinitionsVersion;

pub struct Tosca2_0;

impl ToscaDefinitionsVersion for Tosca2_0 {
    type AttributeDefinition = AttributeDefinition;
    type AttributeAssignment = AttributeAssignment;
    type PropertyAssignment = PropertyAssignment;
    type PropertyDefinition = PropertyDefinition;
    type ParameterDefinition = ParameterDefinition;
    type FileDefinition = ToscaFileDefinition;
    type ImportDefinition = ImportDefinition;
    type SchemaDefinition = SchemaDefinition;
    type RequirementDefinition = RequirementDefinition;
    type RequirementAssignment = RequirementAssignment;
    type DataTypeDefinition = DataType;
    type NodeTypeDefinition = NodeType;
    type NodeTemplateDefinition = NodeTemplate;
    type ServiceTemplateDefinition = ServiceTemplateDefinition;
    type Value = Value;

    fn parse(ctx: &mut toto_ast::AST, n: &yaml_peg::NodeRc) -> toto_ast::GraphHandle {
        // builtin
        let root = ctx.graph.add_node(Entity::File);
        let builtin_url = ctx.graph.add_node(Entity::String("$builtin".to_string()));
        ctx.graph.add_edge(root, builtin_url, Relation::Url);

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("string".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("integer".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("float".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("boolean".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("bytes".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("nil".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("version".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("range".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("timestamp".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("scalar-unit.size".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("list".to_string()));

        let dt = ctx.graph.add_node(Entity::DataType);
        ctx.graph
            .add_edge(root, dt, Relation::Subdef("map".to_string()));

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("$get_input".to_string()));

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("$get_attribute".to_string()));

        let f = ctx.graph.add_node(Entity::Function);
        ctx.graph
            .add_edge(root, f, Relation::Subdef("$get_property".to_string()));

        // TODO: add more builtin entities

        return Self::FileDefinition::parse::<Self>(ctx, n);
    }
}
