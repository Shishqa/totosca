use crate::{add_with_loc, ParseCompatibleEntity, ParseCompatibleRelation, ParseError};

pub type SubfieldParseFn<E, R> =
    fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>);

pub type StaticSchemaMap<E, R> = phf::Map<&'static str, SubfieldParseFn<E, R>>;

pub fn parse_schema<E, R>(
    schema: &StaticSchemaMap<E, R>,
    root: toto_ast::GraphHandle,
    keys: impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)>,
    ast: &mut toto_ast::AST<E, R>,
) -> toto_ast::GraphHandle
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    keys.for_each(|(k, v)| {
        toto_yaml::as_string(k, ast)
            .or_else(|| {
                add_with_loc(ParseError::UnexpectedType("string"), k, ast);
                None
            })
            .and_then(|key| {
                let parser = schema.get(key.as_str());
                if parser.is_some() {
                    parser.unwrap()(root, v, ast);
                } else {
                    add_with_loc(ParseError::UnknownField(key), k, ast);
                }
                Some(k)
            });
    });

    root
}

pub trait Schema<E, R>
where
    E: ParseCompatibleEntity,
    R: ParseCompatibleRelation,
{
    const SCHEMA: StaticSchemaMap<E, R>;
}
