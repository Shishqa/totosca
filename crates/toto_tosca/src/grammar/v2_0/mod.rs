pub mod artifact;
pub mod capability;
pub mod data;
pub mod file;
pub mod function;
pub mod group;
pub mod import;
pub mod interface;
pub mod node;
pub mod notification;
pub mod operation;
pub mod policy;
pub mod relationship;
pub mod requirement;
pub mod service_template;
pub mod substitution_mapping;
pub mod value;
pub mod workflow;

use std::marker::PhantomData;

pub use artifact::*;
pub use capability::*;
pub use data::*;
pub use file::*;
pub use function::*;
pub use group::*;
pub use import::*;
pub use interface::*;
pub use node::*;
pub use notification::*;
pub use operation::*;
pub use policy::*;
pub use relationship::*;
pub use requirement::*;
pub use service_template::*;
pub use substitution_mapping::*;
pub use value::*;
pub use workflow::*;

use super::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion};

pub struct Tosca2_0<E, R>(PhantomData<(E, R)>);

impl<E, R> ToscaDefinitionsVersion for Tosca2_0<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    const NAME: &'static str = "tosca_2_0";

    type Entity = E;
    type Relation = R;
    type FileDefinition = self::ToscaFileDefinition<Self>;
    type ImportDefinition = self::ImportDefinition<Self>;
    type ServiceTemplateDefinition = self::ServiceTemplateDefinition<Self>;
    type SubstitutionMapping = self::SubstitutionMapping<Self>;

    type NodeTypeDefinition = self::NodeTypeDefinition<Self>;
    type NodeTemplateDefinition = self::NodeTemplateDefinition<Self>;
    type DataTypeDefinition = self::DataTypeDefinition<Self>;
    type SchemaDefinition = self::SchemaDefinition<Self>;
    type AttributeDefinition = self::AttributeDefinition<Self>;
    type PropertyDefinition = self::PropertyDefinition<Self>;
    type ParameterDefinition = self::ParameterDefinition<Self>;
    type ArtifactDefinition = self::ArtifactDefinition<Self>;
    type ArtifactTypeDefinition = self::ArtifactTypeDefinition<Self>;
    type CapabilityDefinition = self::CapabilityDefinition<Self>;
    type CapabilityTypeDefinition = self::CapabilityTypeDefinition<Self>;
    type CapabilityAssignment = self::CapabilityAssignment<Self>;
    type GroupDefinition = self::GroupDefinition<Self>;
    type GroupTypeDefinition = self::GroupTypeDefinition<Self>;

    type PolicyDefinition = self::PolicyDefinition<Self>;
    type PolicyTypeDefinition = self::PolicyTypeDefinition<Self>;
    type PolicyTriggerDefinition = self::PolicyTriggerDefinition<Self>;

    type ImplementationDefinition = self::ImplementationDefinition<Self>;
    type InterfaceDefinition = self::InterfaceDefinition<Self>;
    type InterfaceTypeDefinition = self::InterfaceTypeDefinition<Self>;
    type InterfaceAssignment = self::InterfaceAssignment<Self>;
    type OperationDefinition = self::OperationDefinition<Self>;
    type OperationAssignment = self::OperationAssignment<Self>;
    type NotificationDefinition = self::NotificationDefinition<Self>;
    type NotificationAssignment = self::NotificationAssignment<Self>;
    type RelationshipTypeDefinition = self::RelationshipTypeDefinition<Self>;
    type RelationshipTemplateDefinition = self::RelationshipTemplateDefinition<Self>;

    type RequirementDefinition = self::RequirementDefinition<Self>;
    type RequirementAssignment = self::RequirementAssignment<Self>;
    type RelationshipDefinition = self::RelationshipDefinition<Self>;
    type RelationshipAssignment = self::RequirementAssignment<Self>;

    type WorkflowDefinition = self::WorkflowDefinition<Self>;
    type WorkflowStepDefinition = self::WorkflowStepDefinition<Self>;
    type WorkflowActivityDefinition = self::WorkflowActivityDefinition<Self>;

    type FunctionDefinition = self::FunctionDefinition<Self>;
    type FunctionSignatureDefinition = self::FunctionSignatureDefinition<Self>;

    fn add_builtins(
        root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<Self::Entity, Self::Relation>,
    ) {
        const BUILTIN_DATA: &[(&'static str, &'static str)] = &[
            ("string", ""),
            ("integer", ""),
            ("float", ""),
            ("boolean", ""),
            ("bytes", ""),
            ("nil", ""),
            ("timestamp", ""),
            ("scalar-unit", ""),
            ("scalar-unit.time", ""),
            ("version", ""),
            ("list", ""),
            ("map", ""),
        ];

        for (name, details) in BUILTIN_DATA {
            let data_handle =
                ast.add_node(Self::Entity::from(crate::Entity::Data(crate::DataEntity)));
            ast.add_edge(
                root,
                data_handle,
                Self::Relation::from(crate::Relation::Type(crate::TypeRelation(name.to_string()))),
            );

            let description_handle = ast.add_node(Self::Entity::from(toto_yaml::Entity::Str(
                toto_yaml::YamlString(details.to_string()),
            )));
            ast.add_edge(
                data_handle,
                description_handle,
                Self::Relation::from(crate::Relation::Description(crate::DescriptionRelation)),
            );
        }
    }
}

impl<E, R> toto_parser::EntityParser<E, R> for Tosca2_0<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as ToscaDefinitionsVersion>::FileDefinition::parse(n, ast)
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use petgraph::visit::{EdgeFiltered, NodeFiltered, NodeRef};
    use toto_parser::{get_errors, report_error};

    use crate::grammar::tests::{Entity, Relation};
    use crate::{AsToscaEntity, AsToscaRelation, ToscaParser};

    #[test]
    fn tosca_2_0() {
        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_2_0.yaml").unwrap();

        let mut parser = ToscaParser::new();
        parser.parse(&doc_path, &mut ast);

        let tosca_graph = NodeFiltered::from_fn(&ast, |n| ast[n.id()].as_tosca().is_some());
        let tosca_graph = EdgeFiltered::from_fn(&tosca_graph, |e| e.weight().as_tosca().is_some());

        dbg!(Dot::new(&tosca_graph));

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));
    }
}
