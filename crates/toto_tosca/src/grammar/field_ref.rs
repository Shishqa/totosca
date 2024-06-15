use toto_parser::EntityParser;

use crate::{
    semantic::SimpleLookuper, DefinitionRelation, FileEntity, RefRelation, RootRelation,
    ServiceTemplateEntity, ToscaCompatibleEntity, ToscaCompatibleRelation, TypeRelation,
};

use super::v2_0::value;

pub struct FieldRef(pub SimpleLookuper);

impl FieldRef {
    pub fn type_ref<E, R>(entity: E, relation: R) -> Self
    where
        crate::Entity: From<E>,
        crate::Relation: From<R>,
    {
        Self(SimpleLookuper {
            root: (
                crate::Relation::Root(RootRelation),
                crate::Entity::File(FileEntity),
            ),
            what: crate::Entity::from(entity),
            what_rel: |s| crate::Relation::Type(TypeRelation::from(s)),
            then: crate::Relation::from(relation),
        })
    }

    pub fn def_ref<E, R>(entity: E, relation: R) -> Self
    where
        crate::Entity: From<E>,
        crate::Relation: From<R>,
    {
        Self(SimpleLookuper {
            root: (
                crate::Relation::Root(RootRelation),
                crate::Entity::ServiceTemplate(ServiceTemplateEntity),
            ),
            what: crate::Entity::from(entity),
            what_rel: |s| crate::Relation::Definition(DefinitionRelation::from(s)),
            then: crate::Relation::from(relation),
        })
    }

    pub fn parse<E, R>(
        self,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        if let Some(n_handle) = value::StringValue::parse(n, ast) {
            self.link(root, n_handle, ast)
        }
    }

    pub fn link<E, R>(
        self,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        ast.add_edge(
            root,
            n,
            crate::Relation::from(RefRelation {
                lookuper: Box::new(self.0),
            })
            .into(),
        );
    }
}
