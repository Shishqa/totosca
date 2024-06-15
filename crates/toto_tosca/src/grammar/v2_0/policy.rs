use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{
        collection::Collection, field::Field, field_ref::FieldRef, list::List,
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefaultRelation,
    DefinitionRelation, DependencyArtifactRelation, DescriptionRelation, DirectiveRelation,
    EntrySchemaRelation, ExternalSchemaRelation, FileExtRelation, KeySchemaRelation,
    MappingRelation, MetadataRelation, MimeTypeRelation, PolicyTriggerEventRelation,
    PrimaryArtifactRelation, RefHasFileRelation, RefMemberNodeTemplateRelation,
    RefMemberNodeTypeRelation, RefValidRelationshipTypeRelation, RefValidSourceNodeTypeRelation,
    RefValidTargetNodeTypeRelation, RepositoryRelation, RequiredRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, ValidationRelation, ValueRelation, VersionRelation,
    WorkflowActivityRelation,
};

use super::value;

#[derive(Debug)]
pub struct PolicyTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct PolicyDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct PolicyTriggerDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for PolicyTriggerDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::PolicyTriggerEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "event" => Field::<PolicyTriggerEventRelation, value::StringValue>::parse,
        "condition" => |_, _, _| {},
        "action" => List::<WorkflowActivityRelation, V::WorkflowActivityDefinition>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] = &[
        |fields: &HashSet<String>| mandatory(fields, "event"),
        |fields: &HashSet<String>| mandatory(fields, "action"),
    ];
}

impl<E, R, V> toto_parser::Schema<E, R> for PolicyTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::PolicyEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => |r, n, ast| FieldRef::derived_from(crate::Entity::from(crate::PolicyEntity)).parse(r, n, ast),
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "properties" => Collection::<DefinitionRelation, V::PropertyDefinition>::parse,
        "targets" => List::<RefValidTargetNodeTypeRelation, value::StringValue>::parse,
        "triggers" => Collection::<DefinitionRelation, V::PolicyTriggerDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for PolicyDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::PolicyEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => |r, n, ast| FieldRef::has_type(crate::Entity::from(crate::PolicyEntity)).parse(r, n, ast),
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "properties" => Collection::<AssignmentRelation, value::AnyValue>::parse,
        // todo: target nodes and groups
        "targets" => List::<RefValidTargetNodeTypeRelation, value::StringValue>::parse,
        "triggers" => Collection::<DefinitionRelation, V::PolicyTriggerDefinition>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "type")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for PolicyTriggerDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for PolicyTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for PolicyDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as toto_parser::Schema<E, R>>::parse(n, ast)
    }
}
