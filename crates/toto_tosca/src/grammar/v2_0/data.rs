use std::marker::PhantomData;

use toto_parser::{add_with_loc, RelationParser};

use crate::{
    grammar::{collection::Collection, field::Field, ToscaDefinitionsVersion},
    DataEntity, DescriptionRelation, EntrySchemaRelation, KeySchemaRelation, MetadataRelation,
    RefDerivedFromRelation, RefHasTypeRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    VersionRelation,
};

use super::value;

#[derive(Debug)]
pub struct DataTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct SchemaDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<V: ToscaDefinitionsVersion> Default for SchemaDefinition<V> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E, R, V> toto_parser::Schema<E, R> for DataTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => Field::<RefDerivedFromRelation, value::StringValue>::parse,
        "version" => Field::<VersionRelation, value::StringValue>::parse,
        "metadata" => Collection::<MetadataRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "validation" => |_, _, _| todo!(),
        "properties" => |_, _, _| todo!(),
        "key_schema" => Field::<KeySchemaRelation, Self>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, Self>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for SchemaDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::from(crate::DataEntity).into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => Field::<RefHasTypeRelation, value::StringValue>::parse,
        "description" => Field::<DescriptionRelation, value::StringValue>::parse,
        "validation" => |_, _, _| todo!(),
        "key_schema" => Field::<KeySchemaRelation, Self>::parse,
        "entry_schema" => Field::<EntrySchemaRelation, Self>::parse,
    };
}

impl<E, R, V> toto_parser::EntityParser<E, R> for DataTypeDefinition<V>
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

impl<E, R, V> toto_parser::EntityParser<E, R> for SchemaDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let import = add_with_loc(crate::Entity::from(crate::DataEntity), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(import, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| ()).map(|_| {
                ast.add_edge(
                    import,
                    n,
                    crate::Relation::from(crate::RefHasTypeRelation).into(),
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
