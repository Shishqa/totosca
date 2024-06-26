use std::{collections::HashSet, marker::PhantomData};

use toto_parser::{add_with_loc, mandatory, RelationParser};

use crate::{
    grammar::{field::Field, field_ref::DefRef, v2_0, ToscaDefinitionsVersion},
    ToscaCompatibleEntity, ToscaCompatibleRelation,
};

#[derive(Debug)]
pub struct ImportDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> toto_parser::Schema<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::ImportEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "file" => Field::<crate::ImportUrlRelation, v2_0::value::StringValue>::parse,
        "repository" => DefRef::<crate::FileEntity, crate::RepositoryEntity, crate::RepositoryRelation>::parse,
        "namespace_prefix" => Field::<crate::ImportNamespaceRelation, v2_0::value::StringValue>::parse,
    };

    const VALIDATION: &'static [toto_parser::ValidationFieldFn] =
        &[|fields: &HashSet<String>| mandatory(fields, "file")];
}

impl<E, R, V> toto_parser::EntityParser<E, R> for ImportDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let import = add_with_loc(crate::Entity::from(crate::ImportEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(import, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    import,
                    n,
                    crate::Relation::from(crate::ImportUrlRelation).into(),
                );
                import
            }))
            .or_else(|| {
                add_with_loc(
                    toto_parser::ParseError::UnexpectedType("map or string"),
                    n,
                    ast,
                );
                None
            });
        Some(import)
    }
}
