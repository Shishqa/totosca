use std::marker::PhantomData;

use crate::{
    parse::{
        add_error, parse_schema, EntityParser, ParseError, RelationParser, Schema, StaticSchemaMap,
    },
    tosca::{
        ast::{ToscaCompatibleEntity, ToscaCompatibleRelation},
        v2_0, ToscaDefinitionsVersion,
    },
};

#[derive(Debug)]
pub struct ToscaFileDefinition<V: ToscaDefinitionsVersion>(PhantomData<V>);

impl<E, R, V> Schema<E, R> for ToscaFileDefinition<V>
where
    E: ToscaCompatibleEntity,
    R: ToscaCompatibleRelation,
    V: ToscaDefinitionsVersion,
{
    const SCHEMA: StaticSchemaMap<E, R> = phf::phf_map! {
        // "tosca_definitions_version" => |r, n, ast| {
        //     let t =
        //         ast.add_node(toto_tosca::Entity::ToscaDefinitionsVersion.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "profile" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Profile.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        // "metadata" => |r, n, ast| {
        //     let t = ast.add_node(toto_tosca::Entity::Metadata.into());
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        //     ast.add_edge(t, n, ParseLoc.into());
        // },
        "description" => v2_0::value::Field::<v2_0::value::Description, v2_0::value::String>::parse::<E, R>,
        "imports" => v2_0::value::List::<v2_0::import::Import, V::ImportDefinition>::parse::<E, R>,
        // "imports" => |r, n, ast| {
        //     List::<E, R, V::ImportDefinition>(n, r, PhantomData::default())
        //         .parse(ast);
        // },
        // "data_types" => |r, n, ast| {
        //     Collection::<E, R, V::DataTypeDefinition>(
        //         n,
        //         r,
        //         PhantomData::default(),
        //     )
        //     .parse(ast);
        // },
        // "node_types" => |r, n, ast| {
        //     Collection::<E, R, V::NodeTypeDefinition>(
        //         n,
        //         r,
        //         PhantomData::default(),
        //     )
        //     .parse(ast);
        // },
        // "service_template" => |r, n, ast| {
        //     let t = V::ServiceTemplateDefinition::from(n).parse(ast);
        //     ast.add_edge(r, t, toto_tosca::Relation::Subdef.into());
        // }
    };
}

impl<V> EntityParser<toto_ast::GraphHandle> for ToscaFileDefinition<V>
where
    V: ToscaDefinitionsVersion,
{
    fn parse<E, R>(
        n: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle>
    where
        E: ToscaCompatibleEntity,
        R: ToscaCompatibleRelation,
    {
        let file = ast.add_node(toto_tosca::Entity::File.into());
        toto_yaml::as_map(n, ast)
            .or_else(|| {
                add_error(n, ast, ParseError::UnexpectedType("map"));
                None
            })
            .and_then(|items| {
                parse_schema(&Self::SCHEMA, file, items, ast);
                Some(file)
            });
        Some(file)
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;

    use crate::tosca::{
        ast::ToscaParser,
        tests::{Entity, Relation},
    };

    #[test]
    fn it_works() {
        let doc = include_str!("../../../../../tests/tosca_1_3.yaml");

        let mut ast = toto_ast::AST::<Entity, Relation>::new();

        ToscaParser::parse(doc, &mut ast);

        dbg!(Dot::new(&ast));
    }
}
