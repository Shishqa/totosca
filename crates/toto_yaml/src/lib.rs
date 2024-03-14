use std::fmt::Debug;

use petgraph::visit::EdgeRef;

// TODO: move to a separate crate
pub struct FileEntity(pub String);

impl Debug for FileEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.char_indices().nth(100) {
            None => f.write_str(&self.0),
            Some((idx, _)) => f.write_str(&self.0[..idx]),
        }
    }
}

#[derive(Debug)]
pub struct FileRelation(pub usize);

#[derive(Debug, Clone)]
pub struct YamlNull;

#[derive(Debug, Clone)]
pub struct YamlList;

#[derive(Debug, Clone)]
pub struct YamlMap;

#[derive(Debug, Clone)]
pub enum Entity {
    Null(YamlNull),
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(YamlList),
    Map(YamlMap),
}

#[derive(Debug, Clone)]
pub enum Relation {
    MapKey,
    MapValue,
    ListValue(usize),
}

impl From<String> for FileEntity {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&yaml_peg::NodeRc> for Entity {
    fn from(value: &yaml_peg::NodeRc) -> Self {
        match value.yaml() {
            yaml_peg::Yaml::Null => Self::Null(YamlNull),
            yaml_peg::Yaml::Str(v) => Self::Str(v.to_string()),
            yaml_peg::Yaml::Int(_) => Self::Int(value.as_int().unwrap()),
            yaml_peg::Yaml::Bool(v) => Self::Bool(*v),
            yaml_peg::Yaml::Float(_) => Self::Float(value.as_float().unwrap()),
            yaml_peg::Yaml::Seq(_) => Self::List(YamlList),
            yaml_peg::Yaml::Map(_) => Self::Map(YamlMap),
            _ => Self::Null(YamlNull),
        }
    }
}

pub trait AsYamlEntity {
    fn as_yaml(&self) -> Option<&Entity>;
}

pub trait AsYamlRelation {
    fn as_yaml(&self) -> Option<&Relation>;
}

pub struct YamlParser {}

impl YamlParser {
    pub fn parse<E, R>(doc: &str, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle
    where
        E: From<Entity> + From<FileEntity>,
        R: From<Relation> + From<FileRelation>,
    {
        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let doc_handle = ast.add_node(FileEntity(doc.to_string()).into());
        Self::parse_node(yaml, doc_handle, ast)
    }

    fn parse_node<E, R>(
        n: yaml_peg::NodeRc,
        doc_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: From<Entity>,
        R: From<Relation> + From<FileRelation>,
    {
        let node_handle = ast.add_node(Entity::from(&n).into());
        ast.add_edge(
            node_handle,
            doc_handle,
            FileRelation(n.pos() as usize).into(),
        );
        match n.yaml() {
            yaml_peg::Yaml::Map(m) => {
                for (k, v) in m.iter() {
                    let k_handle = Self::parse_node(k.clone(), doc_handle, ast);
                    ast.add_edge(node_handle, k_handle, Relation::MapKey.into());

                    let v_handle = Self::parse_node(v.clone(), doc_handle, ast);
                    ast.add_edge(k_handle, v_handle, Relation::MapValue.into());
                }
            }
            yaml_peg::Yaml::Seq(s) => {
                for (i, v) in s.iter().enumerate() {
                    let v_handle = Self::parse_node(v.clone(), doc_handle, ast);
                    ast.add_edge(node_handle, v_handle, Relation::ListValue(i).into());
                }
            }
            _ => {}
        }
        node_handle
    }
}

pub fn as_map<E: AsYamlEntity, R: AsYamlRelation>(
    n: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> Option<impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)>> {
    match ast[n].as_yaml().unwrap() {
        &Entity::Map(_) => Some(
            ast.edges(n)
                .filter_map(|e| match e.weight().as_yaml() {
                    Some(Relation::MapKey) => Some(e.target()),
                    _ => None,
                })
                .map(|k| {
                    let v = ast
                        .edges(k)
                        .find_map(|e| match e.weight().as_yaml() {
                            Some(Relation::MapValue) => Some(e.target()),
                            _ => None,
                        })
                        .unwrap();

                    (k, v)
                })
                .collect::<Vec<_>>()
                .into_iter(),
        ),
        _ => None,
    }
}

pub fn as_list<E: AsYamlEntity, R: AsYamlRelation>(
    n: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> Option<impl Iterator<Item = (usize, toto_ast::GraphHandle)>> {
    match ast[n].as_yaml().unwrap() {
        Entity::List(_) => Some(
            ast.edges(n)
                .filter_map(|e| match e.weight().as_yaml() {
                    Some(Relation::ListValue(i)) => Some((*i, e.target())),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter(),
        ),
        _ => None,
    }
}

pub fn as_string<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<String>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    let t = &ast[n];
    let t = t.as_yaml().unwrap();
    match t {
        Entity::Str(v) => Some(v.clone()),
        _ => None,
    }
}

pub fn as_null<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<YamlNull>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    let t = &ast[n];
    let t = t.as_yaml().unwrap();
    match t {
        Entity::Null(v) => Some(v.clone()),
        _ => None,
    }
}

pub fn as_bool<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<bool>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    let t = &ast[n];
    let t = t.as_yaml().unwrap();
    match t {
        Entity::Bool(v) => Some(v.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use petgraph::dot::Dot;

    use crate::{FileEntity, FileRelation, YamlParser};

    #[derive(Debug)]
    pub enum Entity {
        File(FileEntity),
        Yaml(crate::Entity),
    }

    #[derive(Debug)]
    pub enum Relation {
        File(FileRelation),
        Yaml(crate::Relation),
    }

    impl From<crate::Entity> for Entity {
        fn from(value: crate::Entity) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<FileEntity> for Entity {
        fn from(value: FileEntity) -> Self {
            Self::File(value)
        }
    }

    impl From<crate::Relation> for Relation {
        fn from(value: crate::Relation) -> Self {
            Self::Yaml(value)
        }
    }

    impl From<FileRelation> for Relation {
        fn from(value: FileRelation) -> Self {
            Self::File(value)
        }
    }

    #[test]
    fn it_works() {
        let doc = include_str!("../../../tests/a.yaml");

        let mut ast = petgraph::Graph::<Entity, Relation, petgraph::Directed, u32>::new();

        YamlParser::parse(doc, &mut ast);

        dbg!(size_of::<Entity>() * ast.node_count() + size_of::<Relation>() * ast.edge_count());
        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
