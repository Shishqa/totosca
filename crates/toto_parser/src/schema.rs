use std::collections::HashSet;

use phf::phf_map;

use crate::{add_with_loc, ParseCompatibleEntity, ParseCompatibleRelation, ParseError};

pub type SubfieldParseFn<E, R> =
    fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>);

pub type StaticSchemaMap<E, R> = phf::Map<&'static str, SubfieldParseFn<E, R>>;

pub type ValidationFieldFn = fn(&HashSet<String>) -> Option<ParseError>;

pub trait Schema<E, R>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    const SELF: fn() -> E;
    const SCHEMA: StaticSchemaMap<E, R> = phf_map!();
    const VALIDATION: &'static [ValidationFieldFn] = &[];

    fn parse_schema(
        root: toto_ast::GraphHandle,
        keys: impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)>,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: ParseCompatibleEntity,
        R: ParseCompatibleRelation,
    {
        let mut parsed_fields = HashSet::<String>::new();
        keys.for_each(|(k, v)| {
            if let Some(key) = toto_yaml::as_string(k, ast).or_else(|| {
                add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                None
            }) {
                parsed_fields.insert(key.clone());
                if let Some(parser) = Self::SCHEMA.get(key.as_str()) {
                    parser(root, v, ast);
                } else {
                    add_with_loc(ParseError::UnknownField(key), k, ast);
                }
            }
        });

        Self::VALIDATION.iter().for_each(|checker| {
            if let Some(err) = checker(&parsed_fields) {
                add_with_loc(err, root, ast);
            };
        });

        root
    }

    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let node_template = add_with_loc(Self::SELF(), n, ast);
        toto_yaml::as_map(n, ast)
            .map(|items| Self::parse_schema(node_template, items, ast))
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("map"), n, ast);
                None
            });
        Some(node_template)
    }
}
