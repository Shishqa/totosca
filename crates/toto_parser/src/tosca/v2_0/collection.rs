use std::marker::PhantomData;

use crate::{
    parse::{add_error, ParseError, ParseLoc},
    tosca::ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
};

pub struct Collection<E, R, P>(
    pub toto_ast::GraphHandle,
    pub toto_ast::GraphHandle,
    pub PhantomData<(E, R, P)>,
)
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;

impl<E, R, P> From<(toto_ast::GraphHandle, toto_ast::GraphHandle)> for Collection<E, R, P>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>,
{
    fn from(value: (toto_ast::GraphHandle, toto_ast::GraphHandle)) -> Self {
        Self(value.0, value.1, PhantomData::default())
    }
}

impl<E, R, P> toto_ast::Parse<E, R> for Collection<E, R, P>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        toto_yaml::as_map(self.0, ast)
            .and_then(|iter| {
                iter.for_each(|(k, v)| {
                    toto_yaml::as_string(k, ast)
                        .or_else(|| {
                            add_error(k, ast, ParseError::UnexpectedType("string"));
                            None
                        })
                        .and_then(|key| {
                            let value = P::from(v).parse(ast);
                            ast.add_edge(
                                self.1,
                                value,
                                toto_tosca::Relation::NamedSubdef(key).into(),
                            );
                            Some(k)
                        });
                });
                Some(self.0)
            })
            .or_else(|| {
                let e = ast.add_node(ParseError::UnexpectedType("map").into());
                ast.add_edge(e, self.0, ParseLoc.into());
                None
            });

        self.1
    }
}
