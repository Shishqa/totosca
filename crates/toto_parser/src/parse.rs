use crate::tosca::{
    ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
    ToscaDefinitionsVersion,
};

#[derive(Debug, Clone)]
pub enum ParseError {
    UnknownField(String),
    MissingField(&'static str),
    UnexpectedType(&'static str),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ParseLoc;

pub fn add_error<E, R>(n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>, err: ParseError)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    let e = ast.add_node(err.into());
    ast.add_edge(e, n, ParseLoc.into());
}

pub(crate) type SubfieldParseFn<E, R> =
    fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>);

pub(crate) type StaticSchemaMap<E, R> = phf::Map<&'static str, SubfieldParseFn<E, R>>;

pub fn parse_schema<E, R>(
    schema: &StaticSchemaMap<E, R>,
    root: toto_ast::GraphHandle,
    keys: impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)>,
    ast: &mut toto_ast::AST<E, R>,
) -> toto_ast::GraphHandle
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    keys.for_each(|(k, v)| {
        toto_yaml::as_string(k, ast)
            .or_else(|| {
                add_error(k, ast, ParseError::UnexpectedType("string"));
                None
            })
            .and_then(|key| {
                let parser = schema.get(key.as_str());
                if parser.is_some() {
                    parser.unwrap()(root, v, ast);
                } else {
                    add_error(k, ast, ParseError::UnknownField(key));
                }
                Some(k)
            });
    });

    root
}

pub trait Schema<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    const SCHEMA: StaticSchemaMap<E, R>;
}

pub trait EntityParser<V> {
    fn parse<E, R>(n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) -> Option<V>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation;
}

pub trait RelationParser {
    fn parse<E, R>(
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation;
}
