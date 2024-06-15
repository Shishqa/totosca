use toto_parser::EntityParser;

use crate::{
    semantic::SimpleLookuper, DerivedFromRelation, FileEntity, HasTypeRelation, RefRelation,
    RootRelation, SubstitutesTypeRelation, ToscaCompatibleEntity, ToscaCompatibleRelation,
    TypeRelation,
};

use super::v2_0::value;

pub struct FieldRef(pub SimpleLookuper);

impl FieldRef {
    pub fn derived_from(entity: crate::Entity) -> Self {
        Self(SimpleLookuper {
            root: (
                crate::Relation::from(RootRelation),
                crate::Entity::from(FileEntity),
            ),
            what: entity,
            what_rel: |s| crate::Relation::from(TypeRelation::from(s)),
            then: crate::Relation::from(DerivedFromRelation),
        })
    }

    pub fn has_type(entity: crate::Entity) -> Self {
        Self(SimpleLookuper {
            root: (
                crate::Relation::from(RootRelation),
                crate::Entity::from(FileEntity),
            ),
            what: entity,
            what_rel: |s| crate::Relation::from(TypeRelation::from(s)),
            then: crate::Relation::from(HasTypeRelation),
        })
    }

    pub fn substitutes_type(entity: crate::Entity) -> Self {
        Self(SimpleLookuper {
            root: (
                crate::Relation::from(RootRelation),
                crate::Entity::from(FileEntity),
            ),
            what: entity,
            what_rel: |s| crate::Relation::from(TypeRelation::from(s)),
            then: crate::Relation::from(SubstitutesTypeRelation),
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
