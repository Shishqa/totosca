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

impl<E, R> toto_ast::EntityParser<E, R> for Tosca1_3<E, R>
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
    use toto_ast::EntityParser;
    use toto_parser::{get_errors, report_error};
    use toto_yaml::YamlParser;

    use crate::grammar::{
        parser::ToscaParser,
        tests::{Entity, Relation},
    };

    #[test]
    fn tosca_1_3() {
        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_1_3.yaml").unwrap();

        let mut doc = toto_yaml::FileEntity::from_url(doc_path);
        doc.fetch().unwrap();
        let doc_handle = ast.add_node(doc.into());

        let doc_root = YamlParser::parse(doc_handle, &mut ast).unwrap();
        ToscaParser::parse(doc_root, &mut ast);

        dbg!(Dot::new(&ast));

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));
        assert!(false)
    }
}
