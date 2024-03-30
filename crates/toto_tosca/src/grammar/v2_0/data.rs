use std::marker::PhantomData;

use toto_parser::{add_with_loc, RelationParser};

use crate::{grammar::ToscaDefinitionsVersion, ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::{value, Description, Metadata, RefDerivedFrom, RefHasType};

#[derive(Debug)]
pub struct DataTypeDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

#[derive(Debug)]
pub struct SchemaDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

pub struct Version;
impl<R> toto_parser::Linker<(), R> for Version
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::Version.into();
}

pub struct KeySchema;
impl<R> toto_parser::Linker<(), R> for KeySchema
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::KeySchema.into();
}

pub struct EntrySchema;
impl<R> toto_parser::Linker<(), R> for EntrySchema
where
    R: ToscaCompatibleRelation,
{
    const L: fn(()) -> R = |_| crate::Relation::EntrySchema.into();
}

impl<E, R, V> toto_parser::Schema<E, R> for DataTypeDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Data.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "derived_from" => toto_parser::Field::<RefDerivedFrom, value::String>::parse,
        "version" => toto_parser::Field::<Version, value::String>::parse,
        "metadata" => toto_parser::Collection::<Metadata, value::String>::parse,
        "description" => toto_parser::Field::<Description, value::String>::parse,
        "validation" => |_, _, _| todo!(),
        "properties" => |_, _, _| todo!(),
        "key_schema" => toto_parser::Field::<KeySchema, Self>::parse,
        "entry_schema" => toto_parser::Field::<EntrySchema, Self>::parse,
    };
}

impl<E, R, V> toto_parser::Schema<E, R> for SchemaDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion<Entity = E, Relation = R>,
{
    const SELF: fn() -> E = || crate::Entity::Data.into();
    const SCHEMA: toto_parser::StaticSchemaMap<E, R> = phf::phf_map! {
        "type" => toto_parser::Field::<RefHasType, value::String>::parse,
        "description" => toto_parser::Field::<Description, value::String>::parse,
        "validation" => |_, _, _| todo!(),
        "key_schema" => toto_parser::Field::<KeySchema, Self>::parse,
        "entry_schema" => toto_parser::Field::<EntrySchema, Self>::parse,
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
        let import = add_with_loc(crate::Entity::Data, n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| <Self as toto_parser::Schema<E, R>>::parse_schema(import, items, ast))
            .or(toto_yaml::as_string(n, ast).map(|_| {
                ast.add_edge(import, n, crate::Relation::RefHasType.into());
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
