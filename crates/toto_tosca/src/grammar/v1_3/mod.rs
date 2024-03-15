use std::marker::PhantomData;

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

use super::ToscaDefinitionsVersion;

pub struct Tosca1_3<E, R>(PhantomData<(E, R)>);

pub mod file;
pub mod import;

pub use file::*;
pub use import::*;

impl<E, R> ToscaDefinitionsVersion for Tosca1_3<E, R>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
{
    type Entity = E;
    type Relation = R;
    type FileDefinition = self::file::ToscaFileDefinition<Self>;
    type ImportDefinition = self::import::ImportDefinition<Self>;
}

impl<E, R> toto_parser::EntityParser<E, R> for Tosca1_3<E, R>
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
    use petgraph::dot::Dot;
    use toto_parser::{get_errors, report_error, EntityParser};
    use toto_yaml::YamlParser;

    use crate::grammar::{
        tests::{Entity, Relation},
        v1_3::Tosca1_3,
    };

    #[test]
    fn tosca_1_3() {
        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_1_3.yaml").unwrap();

        let doc_handle = ast.add_node(toto_yaml::FileEntity::from_url(doc_path).into());

        let doc_root = YamlParser::parse(doc_handle, &mut ast);
        Tosca1_3::parse(doc_root, &mut ast);

        //dbg!(Dot::new(&ast));

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));
        assert!(false)
    }
}
