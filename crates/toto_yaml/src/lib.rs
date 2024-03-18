use std::fmt::Debug;

use anyhow::anyhow;
use petgraph::visit::EdgeRef;

// TODO: move to a separate crate
pub struct FileEntity {
    pub url: url::Url,
    pub content: Option<String>,
}

impl FileEntity {
    pub fn from_url(url: url::Url) -> Self {
        Self { url, content: None }
    }

    pub fn fetch(&mut self) -> anyhow::Result<()> {
        let path = self.url.to_file_path();
        if path.is_err() {
            return Err(anyhow!("only local paths are supported"));
        }
        self.content = Some(std::fs::read_to_string(path.unwrap())?);
        Ok(())
    }
}

impl Debug for FileEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.content {
            Some(content) => match content.char_indices().nth(100) {
                None => f.write_str(content),
                Some((idx, _)) => f.write_str(&content[..idx]),
            },
            None => f.write_str("not loaded"),
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

pub trait AsFileEntity {
    fn as_file(&self) -> Option<&FileEntity>;
}

pub trait AsFileRelation {
    fn as_file(&self) -> Option<&FileRelation>;
}

pub struct YamlParser;

impl YamlParser {
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

impl<E, R> toto_ast::EntityParser<E, R> for YamlParser
where
    E: AsFileEntity + From<Entity>,
    R: From<Relation> + From<FileRelation>,
{
    fn parse(
        doc_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Option<toto_ast::GraphHandle> {
        let doc = ast.node_weight(doc_handle).unwrap().as_file().unwrap();
        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc.content.as_ref().unwrap());
        if yaml.is_err() {
            return None;
        }

        Some(Self::parse_node(yaml.unwrap().remove(0), doc_handle, ast))
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
        Entity::Bool(v) => Some(*v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use petgraph::dot::Dot;
    use toto_ast::EntityParser;

    use crate::{AsFileEntity, FileEntity, FileRelation, YamlParser};

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

    impl AsFileEntity for Entity {
        fn as_file(&self) -> Option<&FileEntity> {
            match self {
                Self::File(f) => Some(f),
                Self::Yaml(_) => None,
            }
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
        let mut ast = petgraph::Graph::<Entity, Relation, petgraph::Directed, u32>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/a.yaml").unwrap();

        dbg!(&doc_path);

        let mut doc = FileEntity::from_url(doc_path);
        doc.fetch().unwrap();
        let doc_handle = ast.add_node(doc.into());

        YamlParser::parse(doc_handle, &mut ast).unwrap();

        dbg!(size_of::<Entity>() * ast.node_count() + size_of::<Relation>() * ast.edge_count());
        dbg!(Dot::new(&ast));
    }
}
