use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, ParseError, RelationParser, Schema};

use crate::{
    grammar::{
        collection::Collection,
        field::Field,
        list::{KeyedList, List},
        ToscaDefinitionsVersion,
    },
    AssignmentRelation, ChecksumAlgorithmRelation, ChecksumRelation, DefaultRelation,
    DefinitionRelation, DependencyArtifactRelation, DescriptionRelation, DirectiveRelation,
    EntrySchemaRelation, ExternalSchemaRelation, FileExtRelation, KeySchemaRelation,
    MappingRelation, MetadataRelation, MimeTypeRelation, OrderedDefinitionRelation,
    PrimaryArtifactRelation, RefDerivedFromRelation, RefHasFileRelation, RefHasTypeRelation,
    RefMemberNodeTemplateRelation, RefMemberNodeTypeRelation, RefValidRelationshipTypeRelation,
    RefValidSourceNodeTypeRelation, RepositoryRelation, RequiredRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation, ValidationRelation, ValueRelation, VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct SubstitutionMapping<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for SubstitutionMapping<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::SubstitutionMappingEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "node_type" => Field::<RefHasTypeRelation, value::StringValue>::parse,
        "substitution_filter" => |_, _, _| {},
        "properties" => Collection::<DefinitionRelation, value::AnyValue>::parse,
        "attributes" => Collection::<DefinitionRelation, value::AnyValue>::parse,
        "capabilities" => Collection::<DefinitionRelation, value::AnyValue>::parse,
        "requirements" => KeyedList::<OrderedDefinitionRelation, value::AnyValue>::parse,
        "interfaces" => Collection::<DefinitionRelation, value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "node_type")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for SubstitutionMapping<V>
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
