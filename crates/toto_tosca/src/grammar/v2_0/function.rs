use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefaultRelation,
    DefinitionRelation, DependencyArtifactRelation, DescriptionRelation, DirectiveRelation,
    EntrySchemaRelation, ExternalSchemaRelation, FileExtRelation, FunctionArgumentRelation,
    FunctionOptionalArgumentRelation, FunctionSignatureRelation, KeySchemaRelation,
    MappingRelation, MetadataRelation, MimeTypeRelation, PrimaryArtifactRelation,
    RefDerivedFromRelation, RefHasFileRelation, RefHasTypeRelation, RefMemberNodeTemplateRelation,
    RefMemberNodeTypeRelation, RefValidRelationshipTypeRelation, RefValidSourceNodeTypeRelation,
    RepositoryRelation, RequiredRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    ValidationRelation, ValueRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct FunctionSignatureDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct FunctionDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for FunctionSignatureDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::FunctionSignatureEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "arguments" => List::<FunctionArgumentRelation, V::SchemaDefinition>::parse,
        "optional_arguments" => List::<FunctionOptionalArgumentRelation, V::SchemaDefinition>::parse,
        "variadic" => Field::<DefinitionRelation, value::BoolValue>::parse,
        "result" => Field::<DefinitionRelation, V::SchemaDefinition>::parse,
        "implementation" => Field::<DefinitionRelation, V::ImplementationDefinition>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for FunctionDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::FunctionEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "signatures" => List::<FunctionSignatureRelation, V::FunctionSignatureDefinition>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "signatures")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for FunctionSignatureDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for FunctionDefinition<V>
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
