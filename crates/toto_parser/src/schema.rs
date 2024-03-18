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
            toto_yaml::as_string(k, ast)
                .or_else(|| {
                    add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                    None
                })
                .and_then(|key| {
                    parsed_fields.insert(key.clone());
                    let parser = Self::SCHEMA.get(key.as_str());
                    if parser.is_some() {
                        parser.unwrap()(root, v, ast);
                    } else {
                        add_with_loc(ParseError::UnknownField(key), k, ast);
                    }
                    Some(k)
                });
        });

        Self::VALIDATION.into_iter().for_each(|checker| {
            checker(&parsed_fields).and_then(|err| {
                add_with_loc(err, root, ast);
                Some(())
            });
        });

        root
    }
}
