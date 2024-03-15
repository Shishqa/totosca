pub mod file;
pub mod import;
pub mod value;

use std::marker::PhantomData;

pub use file::*;
pub use import::*;
pub use value::*;

use super::{ToscaCompatibleEntity, ToscaCompatibleRelation, ToscaDefinitionsVersion};

pub struct Tosca2_0<E, R>(PhantomData<(E, R)>);

impl<E, R> ToscaDefinitionsVersion for Tosca2_0<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    type Entity = E;
    type Relation = R;
    type FileDefinition = self::ToscaFileDefinition<Self>;
    type ImportDefinition = self::ImportDefinition<Self>;
}

impl<E, R> toto_parser::EntityParser<E, R> for Tosca2_0<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    fn parse(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        <Self as ToscaDefinitionsVersion>::FileDefinition::parse(n, ast)
    }
}

#[cfg(test)]
mod tests {
    use toto_parser::{get_errors, report_error, EntityParser};
    use toto_yaml::YamlParser;

    use crate::grammar::{
        tests::{Entity, Relation},
        v2_0::Tosca2_0,
    };

    #[test]
    fn tosca_2_0() {
        let doc = include_str!("../../../../../tests/tosca_2_0.yaml");

        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_root = YamlParser::parse(doc, &mut ast);
        Tosca2_0::parse(doc_root, &mut ast);

        // dbg!(Dot::new(&ast));

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));
        assert!(false)
    }
}
