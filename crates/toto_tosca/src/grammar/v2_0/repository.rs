use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, RelationParser};

use crate::{
    grammar::{collection::Collection, field::Field, ToscaDefinitionsVersion},
    DescriptionRelation, MetadataRelation, RepositoryUrlRelation, ToscaCompatibleEntity,
    ToscaCompatibleRelation,
};

use super::value::{self};

#[derive(Debug)]
pub struct RepositoryDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for RepositoryDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::RepositoryEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "url" => Field::<RepositoryUrlRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::AnyValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "url")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for RepositoryDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        match ast.node_weight(n).unwrap().as_yaml() {
            Some(toto_yaml::Entity::Map(_)) => <Self as toto_parser::Schema<E, R>>::parse(n, ast),
            Some(toto_yaml::Entity::Str(_) | toto_yaml::Entity::Null(_)) => {
                let repo = add_with_loc(crate::Entity::from(crate::RepositoryEntity), n, ast);
                ast.add_edge(
                    repo,
                    n,
                    crate::Relation::from(crate::RepositoryUrlRelation).into(),
                );
                Some(repo)
            }
            _ => {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            }
        }
    }
}
