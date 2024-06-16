use std::marker::PhantomData;

use toto_parser::EntityParser;

use crate::{
    semantic::SimpleLookuper, DefinitionRelation, FileEntity, RefRelation, RootRelation,
    ToscaCompatibleEntity, ToscaCompatibleRelation, TypeRelation,
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

    pub fn parse<E, R>(
        self,
        root: toto_ast::GraphHandle,
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        if let Some(n_handle) = value::NullableStringValue::parse(n, ast) {
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

pub struct DefRef<Where, What, Then>(PhantomData<(Where, What, Then)>);

impl<Where, What, Then, E, R> toto_parser::RelationParser<E, R> for DefRef<Where, What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    Where: Default,
    What: Default,
    Then: Default,
    crate::Entity: From<What> + From<Where>,
    crate::Relation: From<Then>,
{
    fn parse(root: toto_ast::GraphHandle, n: toto_ast::GraphHandle, ast: &mut toto_ast::AST<E, R>) {
        FieldRef(SimpleLookuper {
            root: (
                crate::Relation::Root(RootRelation),
                crate::Entity::from(Where::default()),
            ),
            what: crate::Entity::from(What::default()),
            what_rel: |s| crate::Relation::Definition(DefinitionRelation::from(s)),
            then: crate::Relation::from(Then::default()),
        })
        .parse(root, n, ast)
    }
}

impl<Where, What, Then, E, R> toto_parser::ValueRelationParser<E, R, usize>
    for DefRef<Where, What, Then>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    Where: Default,
    What: Default,
    Then: Default,
    crate::Entity: From<What> + From<Where>,
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
