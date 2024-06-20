use std::marker::PhantomData;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{v2_0, ToscaDefinitionsVersion};

pub struct Tosca1_3<E, R>(PhantomData<(E, R)>);

pub mod artifact;
pub mod data;
pub mod file;
pub mod import;
pub mod relationship;

pub use artifact::*;
pub use data::*;
pub use file::*;
pub use import::*;
pub use relationship::*;

impl<E, R> ToscaDefinitionsVersion for Tosca1_3<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    const NAME: &'static str = "tosca_simple_yaml_1_3";

    type Entity = E;
    type Relation = R;
    type FileDefinition = self::file::ToscaFileDefinition<Self>;
    type ImportDefinition = self::import::ImportDefinition<Self>;
    type ServiceTemplateDefinition = v2_0::ServiceTemplateDefinition<Self>;
    type NodeTypeDefinition = v2_0::NodeTypeDefinition<Self>;
    type NodeTemplateDefinition = v2_0::NodeTemplateDefinition<Self>;
    type DataTypeDefinition = self::DataTypeDefinition<Self>;
    type SchemaDefinition = self::SchemaDefinition<Self>;
    type AttributeDefinition = self::AttributeDefinition<Self>;
    type PropertyDefinition = self::PropertyDefinition<Self>;
    type ParameterDefinition = self::ParameterDefinition<Self>;
    type ImplementationDefinition = self::ImplementationDefinition<Self>;
    type ArtifactDefinition = self::ArtifactDefinition<Self>;
    type ArtifactTypeDefinition = v2_0::ArtifactTypeDefinition<Self>;
    type PolicyTriggerDefinition = v2_0::PolicyTriggerDefinition<Self>;
    type PolicyTypeDefinition = v2_0::PolicyTypeDefinition<Self>;
    type PolicyDefinition = v2_0::PolicyDefinition<Self>;
    type GroupDefinition = v2_0::GroupDefinition<Self>;
    type GroupTypeDefinition = v2_0::GroupTypeDefinition<Self>;
    type WorkflowDefinition = v2_0::WorkflowDefinition<Self>;
    type WorkflowStepDefinition = v2_0::WorkflowStepDefinition<Self>;
    type WorkflowActivityDefinition = v2_0::WorkflowActivityDefinition<Self>;
    type SubstitutionMapping = v2_0::SubstitutionMapping<Self>;
    type CapabilityDefinition = v2_0::CapabilityDefinition<Self>;
    type CapabilityAssignment = v2_0::CapabilityAssignment<Self>;
    type CapabilityTypeDefinition = v2_0::CapabilityTypeDefinition<Self>;
    type InterfaceAssignment = v2_0::InterfaceAssignment<Self>;
    type InterfaceDefinition = v2_0::InterfaceDefinition<Self>;
    type InterfaceTypeDefinition = v2_0::InterfaceTypeDefinition<Self>;
    type OperationDefinition = v2_0::OperationDefinition<Self>;
    type OperationAssignment = v2_0::OperationAssignment<Self>;
    type NotificationDefinition = v2_0::NotificationDefinition<Self>;
    type NotificationAssignment = v2_0::NotificationAssignment<Self>;
    type RequirementDefinition = v2_0::RequirementDefinition<Self>;
    type RequirementAssignment = v2_0::RequirementAssignment<Self>;
    type RelationshipDefinition = v2_0::RelationshipDefinition<Self>;
    type RelationshipAssignment = v2_0::RelationshipAssignment<Self>;
    type RelationshipTypeDefinition = self::RelationshipTypeDefinition<Self>;
    type RelationshipTemplateDefinition = v2_0::RelationshipTemplateDefinition<Self>;

    type FunctionDefinition = v2_0::AnyValue;
    type FunctionSignatureDefinition = v2_0::AnyValue;

    fn add_builtins(
        root: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<Self::Entity, Self::Relation>,
    ) {
        const BUILTIN_DATA: &[(&str, &str)] = &[
            ("string", ""),
            ("integer", ""),
            ("float", ""),
            ("boolean", ""),
            ("bytes", ""),
            ("timestamp", ""),
            ("null", ""),
            ("scalar-unit", ""),
            ("scalar-unit.time", ""),
            ("scalar-unit.size", ""),
            ("scalar-unit.frequency", ""),
            ("scalar-unit.bitrate", ""),
            ("version", ""),
            ("range", ""),
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

impl<E, R> toto_parser::EntityParser<E, R> for Tosca1_3<E, R>
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
    use toto_parser::{get_errors, report_error};

    use crate::grammar::tests::{Entity, Relation};
    use crate::ToscaParser;

    #[test]
    fn tosca_1_3() {
        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_1_3.yaml").unwrap();

        let mut parser = ToscaParser::new();
        parser.parse(&doc_path, &mut ast).unwrap();

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));
    }
}
