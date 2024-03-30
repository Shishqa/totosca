pub mod file;
pub mod import;
pub mod node;
pub mod service_template;
pub mod value;

use std::marker::PhantomData;

pub use file::*;
pub use import::*;
pub use node::*;
pub use service_template::*;
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
    type ServiceTemplateDefinition = self::ServiceTemplateDefinition<Self>;
    type NodeTypeDefinition = self::NodeTypeDefinition<Self>;
    type NodeTemplateDefinition = self::NodeTemplateDefinition<Self>;
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
    use petgraph::dot::Dot;
    use petgraph::visit::{EdgeFiltered, NodeFiltered, NodeRef};
    use toto_parser::{get_errors, report_error};

    use crate::grammar::tests::{Entity, Relation};
    use crate::{AsToscaEntity, AsToscaRelation, ToscaParser};

    #[test]
    fn tosca_2_0() {
        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_2_0.yaml").unwrap();

        let mut parser = ToscaParser::new();
        parser.parse(&doc_path, &mut ast);

        let tosca_graph =
            NodeFiltered::from_fn(&ast, |n| matches!(ast[n.id()].as_tosca(), Some(_)));
        let tosca_graph =
            EdgeFiltered::from_fn(&tosca_graph, |e| matches!(e.weight().as_tosca(), Some(_)));

        dbg!(Dot::new(&tosca_graph));

        get_errors(&ast).for_each(|(what, loc)| report_error(what, loc, &ast));

        assert!(false);
    }
}
