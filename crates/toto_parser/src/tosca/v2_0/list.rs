use std::marker::PhantomData;

use crate::{
    parse::{self, add_error, Parse, ParseError, ParseLoc},
    tosca::ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
};

pub struct List<I>;

impl<I, RelationFactory> Parse for List<I, RelationFactory>
where
    I: Parse,
{
    fn with_relation_factory

    fn parse<E, R, V>(
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        toto_yaml::as_list(n, ast)
            .or_else(|| {
                add_error(n, ast, ParseError::UnexpectedType("list").into());
                None
            })
            .and_then(|iter| {
                iter.for_each(|(order, v)| {
                    let value = I::parse(root, n, ast)
                    ast.add_edge(
                        self.1,
                        value,
                        R
                        toto_tosca::Relation::OrderedSubdef(order).into(),
                    );
                });
                Some(self.0)
            })

        self.1
    }

    // fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
    // }
}

// pub struct KeyedList<E, R, P>(
//     pub toto_ast::GraphHandle,
//     pub toto_ast::GraphHandle,
//     pub PhantomData<(E, R, P)>,
// )
// where
//     E: ToscaCompatibleEntity,
//     R: ToscaCompatibleRelation,
//     P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>;
//
// impl<E, R, P> toto_ast::Parse<E, R> for KeyedList<E, R, P>
// where
//     E: ToscaCompatibleEntity,
//     R: ToscaCompatibleRelation,
//     P: toto_ast::Parse<E, R> + From<toto_ast::GraphHandle>,
// {
//     fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
//         let t = ast.node_weight(self.0).unwrap();
//         let t = t.as_yaml().unwrap();
//
//         toto_yaml::as_list(self.0, ast)
//             .and_then(|iter| {
//                 iter.collect::<Vec<_>>().iter().for_each(|(order, item)| {
//                     if let Some(iter) = toto_yaml::as_map(*item, ast) {
//                         let keys = iter.collect::<Vec<_>>();
//                         if keys.len() != 1 {
//                             add_error(
//                                 *item,
//                                 ast,
//                                 ParseError::Custom("should have only one key".to_string()),
//                             );
//                             return;
//                         }
//
//                         let (k, v) = keys.iter().next().unwrap();
//                         let key = ast
//                             .node_weight(*k)
//                             .unwrap()
//                             .as_yaml()
//                             .map(|key| match key {
//                                 toto_yaml::Entity::Str(str_key) => Some(str_key.to_string()),
//                                 _ => None,
//                             })
//                             .unwrap();
//
//                         if let None = key {
//                             add_error(*k, ast, ParseError::UnexpectedType("string"));
//                             return;
//                         }
//
//                         let value = P::from(*v).parse(ast);
//                         ast.add_edge(
//                             self.1,
//                             value,
//                             toto_tosca::Relation::NamedSubdef(key.unwrap()).into(),
//                         );
//                         ast.add_edge(
//                             self.1,
//                             value,
//                             toto_tosca::Relation::OrderedSubdef(*order).into(),
//                         );
//                     } else {
//                         let e = ast.add_node(ParseError::UnexpectedType("map").into());
//                         ast.add_edge(e, *item, ParseLoc.into());
//                     }
//                 });
//                 Some(self.0)
//             })
//             .or_else(|| {
//                 let e = ast.add_node(ParseError::UnexpectedType("list").into());
//                 ast.add_edge(e, self.0, ParseLoc.into());
//                 None
//             });
//
//         self.1
//     }
// }
