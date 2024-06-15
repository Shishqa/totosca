use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, EntityParser, ParseError, RelationParser, Schema};

use crate::{
    grammar::{collection::Collection, field::Field, list::List, ToscaDefinitionsVersion},
    DefinitionRelation, DescriptionRelation, MetadataRelation,
    RefTargetNodeRelation, ToscaCompatibleEntity, ToscaCompatibleRelation, WorkflowActivityRelation,
};

use super::value;

#[derive(Debug)]
pub struct WorkflowDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowStepDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowActivityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowDelegateActivityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowInlineActivityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowSetStateActivityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct WorkflowCallOperationActivityDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for WorkflowDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::WorkflowEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "inputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "outputs" => Collection::<DefinitionRelation, V::ParameterDefinition>::parse,
        "precondition" => |_, _, _| {},
        "steps" => Collection::<DefinitionRelation, V::WorkflowStepDefinition>::parse,
        "implementation" => Field::<DefinitionRelation, V::ImplementationDefinition>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] = &[
        |fields: &HashSet<String>| {
            if fields.contains("steps") && fields.contains("implementation") {
                Some(toto_parser::ParseError::Custom(
                    "steps and implementation are mutually exclusive".to_string(),
                ))
            } else {
                None
            }
        },
        |fields: &HashSet<String>| {
            if !fields.contains("steps") && !fields.contains("implementation") {
                Some(toto_parser::ParseError::MissingField(
                    "steps or implementation",
                ))
            } else {
                None
            }
        },
    ];
}

impl<E, R, V> toto_parser::Schema<E, R> for WorkflowStepDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::WorkflowStepEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "target" => Field::<RefTargetNodeRelation, value::StringValue>::parse,
        "target_relationship" => Field::<DefinitionRelation, value::StringValue>::parse,
        "filter" => |_, _, _| {},
        "activities" => List::<WorkflowActivityRelation, V::WorkflowActivityDefinition>::parse,
        "on_success" => |_, _, _| {},
        "on_failure" => |_, _, _| {},
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] = &[
        |fields: &HashSet<String>| mandatory(fields, "target"),
        |fields: &HashSet<String>| mandatory(fields, "activities"),
    ];
}

impl<E, R, V> toto_parser::Schema<E, R> for WorkflowDelegateActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::WorkflowDelegateActivityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "workflow" => Field::<DescriptionRelation, value::StringValue>::parse,
        "inputs" => Collection::<DefinitionRelation, value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "workflow")];
}

impl<E, R, V> toto_parser::Schema<E, R> for WorkflowInlineActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::WorkflowInlineActivityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "workflow" => Field::<DescriptionRelation, value::StringValue>::parse,
        "inputs" => Collection::<DefinitionRelation, value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "workflow")];
}

impl<E, R, V> toto_parser::Schema<E, R> for WorkflowCallOperationActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E =
        || crate::Entity::from(crate::WorkflowCallOperationActivityEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "operation" => Field::<DescriptionRelation, value::StringValue>::parse,
        "inputs" => Collection::<DefinitionRelation, value::AnyValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "operation")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowStepDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowDelegateActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let activity = add_with_loc(
            crate::Entity::from(crate::WorkflowDelegateActivityEntity),
            n,
            ast,
        );
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(activity, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    activity,
                    n,
                    crate::Relation::from(crate::RefWorkflowRelation).into(),
                );
                activity
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(activity)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowInlineActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let activity = add_with_loc(
            crate::Entity::from(crate::WorkflowInlineActivityEntity),
            n,
            ast,
        );
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(activity, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    activity,
                    n,
                    crate::Relation::from(crate::RefWorkflowRelation).into(),
                );
                activity
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(activity)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowCallOperationActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let activity = add_with_loc(
            crate::Entity::from(crate::WorkflowCallOperationActivityEntity),
            n,
            ast,
        );
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(activity, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    activity,
                    n,
                    crate::Relation::from(crate::RefOperationRelation).into(),
                );
                activity
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(activity)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowSetStateActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let activity = add_with_loc(
            crate::Entity::from(crate::WorkflowSetStateActivityEntity),
            n,
            ast,
        );
        toto_yaml::as_string(n, ast)
            .map(|_| ())
            .map(|_| {
                ast.add_edge(
                    activity,
                    n,
                    crate::Relation::from(crate::DefinitionRelation::default()).into(),
                );
                activity
            })
            .or_else(|| {
                add_with_loc(toto_parser::ParseError::UnexpectedType("string"), n, ast);
                None
            });
        Some(activity)
    }
}

impl<E, R, V> toto_parser::EntityParser<E, R> for WorkflowActivityDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        toto_yaml::as_map(n, ast)
            .or_else(|| {
                add_with_loc(toto_parser::ParseError::UnexpectedType("map"), n, ast);
                None
            })
            .and_then(|items| {
                let mut items = items.take(2);

                if let Some((k, v)) = items.next() {
                    let k_str = toto_yaml::as_string(k, ast)
                        .map(|s| s.0.as_str());

                    if k_str.is_none() {
                        add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                        return None;
                    }

                    let activity = match k_str {
                        Some("delegate") => {
                            <WorkflowDelegateActivityDefinition<V> as EntityParser<E, R>>::parse(
                                v, ast,
                            )
                        }
                        Some("inline") => <WorkflowInlineActivityDefinition<V> as EntityParser<
                            E,
                            R,
                        >>::parse(v, ast),
                        Some("set_state") => {
                            <WorkflowSetStateActivityDefinition<V> as EntityParser<E, R>>::parse(
                                v, ast,
                            )
                        }
                        Some("call_operation") => {
                            <WorkflowCallOperationActivityDefinition<V> as EntityParser<E, R>>::parse(
                                v, ast,
                            )
                        }
                        _ => None,
                    };

                    if activity.is_none() {
                        add_with_loc(ParseError::Custom("unknown activity kind".to_string()), k, ast);
                    }

                    return activity;

                } else {
                    add_with_loc(ParseError::Custom("expected a key".to_string()), n, ast);
                }

                if items.next().is_some() {
                    add_with_loc(
                        ParseError::Custom("expected only one key".to_string()),
                        n,
                        ast,
                    );
                }

                None
            })
    }
}
