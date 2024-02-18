use crate::tosca::ast::{ToscaCompatibleEntity, ToscaCompatibleRelation};

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

pub trait StaticSchema<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    const ROOT: toto_tosca::Entity;
    const SCHEMA: phf::Map<
        &'static str,
        fn(toto_ast::GraphHandle, toto_ast::GraphHandle, &mut toto_ast::AST<E, R>),
    >;

    fn parse_schema(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle {
        let root = ast.add_node(toto_tosca::Entity::File.into());
        ast.add_edge(root, n, ParseLoc.into());

        toto_yaml::iter_keys(n, ast)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(k, v)| {
                let key = ast.node_weight(*k).unwrap().as_yaml().unwrap();
                if let toto_yaml::Entity::Str(str_key) = key {
                    let parser = Self::SCHEMA.get(str_key);
                    if parser.is_some() {
                        parser.unwrap()(root, *v, ast);
                    } else {
                        add_error(*k, ast, ParseError::UnknownField(str_key.to_string()));
                    }
                } else {
                    add_error(*k, ast, ParseError::UnexpectedType("string"));
                }
            });

        root
    }
}
