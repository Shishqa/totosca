use petgraph::{data::DataMap, visit::Dfs};

use crate::{ToscaCompatibleEntity, ToscaCompatibleRelation};

pub struct Hierarchy;

impl Hierarchy {
    pub fn link<E, R>(
        file_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let mut dfs = Dfs::new(&*ast, file_handle);
        while let Some(nx) = dfs.next(&*ast) {
            if ast.node_weight(nx).unwrap().as_tosca().is_some() {
                ast.add_edge(file_handle, nx, crate::Relation::RefRoot.into());
            }
        }
        file_handle
    }
}
