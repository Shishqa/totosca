use std::{collections::HashSet, marker::PhantomData};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    AssignmentRelation, DefinitionRelation, DescriptionRelation, DirectiveRelation,
    MetadataRelation, RefTargetCapabilityRelation, RefTargetNodeRelation,
    RefValidCapabilityTypeRelation, RefValidTargetNodeTypeRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};
use toto_parser::{add_with_loc, mandatory, RelationParser};

use super::value;

#[derive(Debug)]
pub struct RequirementDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct RequirementAssignment<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for RequirementDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RequirementEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "relationship" => Field::<DefinitionRelation, V::RelationshipDefinition>::parse,
        "node" => Field::<RefValidTargetNodeTypeRelation, value::StringValue>::parse,
        "capability" => Field::<RefValidCapabilityTypeRelation, value::StringValue>::parse,
        "node_filter" => |_, _, _| {},
        "count_range" => |_, _, _| {},
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "relationship")];
}

impl<E, R, V> toto_parser::Schema<E, R> for RequirementAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RequirementEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "node" => Field::<RefTargetNodeRelation, value::StringValue>::parse,
        "capability" => Field::<RefTargetCapabilityRelation, value::StringValue>::parse,
        "relationship" => Field::<AssignmentRelation, V::RelationshipAssignment>::parse,
        "allocation" => |_, _, _| {},
        "count" => |_, _, _| {},
        "node_filter" => |_, _, _| {},
        "directives" => List::<DirectiveRelation, value::StringValue>::parse,
        "optional" => |_, _, _| {},
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for RequirementDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for RequirementAssignment<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let req = add_with_loc(crate::Entity::from(crate::RequirementEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(req, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    req,
                    n,
                    crate::Relation::from(crate::RefTargetNodeRelation).into(),
                );
                req
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(req)
    }
}
