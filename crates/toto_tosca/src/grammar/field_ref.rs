use std::marker::PhantomData;

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

pub struct TypeRef<What, Then>(PhantomData<(What, Then)>);

impl<What, Then, E, R> toto_parser::RelationParser<E, R> for TypeRef<What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    What: Default,
    Then: Default,
    crate::Entity: From<What>,
    crate::Relation: From<Then>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        FieldRef::type_ref(What::default(), Then::default()).parse(root, n, ast)
    }
}

impl<What, Then, E, R> toto_parser::ValueRelationParser<E, R, usize> for TypeRef<What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    What: Default,
    Then: Default,
    crate::Entity: From<What>,
    crate::Relation: From<Then>,
{
    fn parse(
        _: usize,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) {
        <Self as toto_parser::RelationParser<E, R>>::parse(root, n, ast);
    }
}

pub struct DefRef<What, Then>(PhantomData<(What, Then)>);

impl<What, Then, E, R> toto_parser::RelationParser<E, R> for DefRef<What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    What: Default,
    Then: Default,
    crate::Entity: From<What>,
    crate::Relation: From<Then>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        FieldRef::def_ref(What::default(), Then::default()).parse(root, n, ast)
    }
}

impl<What, Then, E, R> toto_parser::ValueRelationParser<E, R, usize> for DefRef<What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    What: Default,
    Then: Default,
    crate::Entity: From<What>,
    crate::Relation: From<Then>,
{
    fn parse(
        _: usize,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) {
        <Self as toto_parser::RelationParser<E, R>>::parse(root, n, ast);
    }
}
