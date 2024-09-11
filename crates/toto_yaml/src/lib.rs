extern crate derive_more;
use derive_more::{From, TryInto};
use yaml_parser::ast::AstNode;

use std::fmt::Debug;

use anyhow::anyhow;
use petgraph::{data::DataMap, visit::EdgeRef};

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
pub struct YamlBool(pub bool);

#[derive(Debug, Clone)]
pub struct YamlInt(pub i64);

#[derive(Debug, Clone)]
pub struct YamlFloat(pub f64);

#[derive(Debug, Clone)]
pub struct YamlString(pub String);

#[derive(Debug, Clone)]
pub struct YamlList;

#[derive(Debug, Clone)]
pub struct YamlMap;

#[derive(Debug, Clone, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Entity {
    Null(YamlNull),
    Bool(YamlBool),
    Int(YamlInt),
    Float(YamlFloat),
    Str(YamlString),
    List(YamlList),
    Map(YamlMap),
}

#[derive(Debug, Clone)]
pub struct YamlMapKey;

#[derive(Debug, Clone)]
pub struct YamlMapValue;

#[derive(Debug, Clone)]
pub struct YamlListValue(usize);

#[derive(Debug, Clone, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum Relation {
    MapKey(YamlMapKey),
    MapValue(YamlMapValue),
    ListValue(YamlListValue),
}

impl From<&yaml_parser::ast::Block> for Entity {
    fn from(value: &yaml_parser::ast::Block) -> Self {
        if let Some(scalar) = value.block_scalar() {
            return Self::Str(YamlString(scalar.text().unwrap().text().to_string()));
        } else if value.block_map().is_some() {
            return Self::Map(YamlMap);
        } else if value.block_seq().is_some() {
            return Self::List(YamlList);
        }
        Self::Null(YamlNull)
    }
}

impl From<&yaml_parser::ast::Flow> for Entity {
    fn from(value: &yaml_parser::ast::Flow) -> Self {
        if let Some(scalar) = value.plain_scalar() {
            if scalar.text() == "null" {
                return Self::Null(YamlNull);
            } else if let Ok(b) = scalar.text().parse::<bool>() {
                return Self::Bool(YamlBool(b));
            } else if let Ok(f) = scalar.text().parse::<f64>() {
                return Self::Float(YamlFloat(f));
            } else if let Ok(i) = scalar.text().parse::<i64>() {
                return Self::Int(YamlInt(i));
            }
            return Self::Str(YamlString(scalar.text().to_string()));
        } else if let Some(scalar) = value.single_quoted_scalar() {
            return Self::Str(YamlString(scalar.text().to_string()));
        } else if let Some(scalar) = value.double_qouted_scalar() {
            return Self::Str(YamlString(scalar.text().to_string()));
        } else if value.flow_map().is_some() {
            return Self::Map(YamlMap);
        } else if value.flow_seq().is_some() {
            return Self::List(YamlList);
        }
        Self::Null(YamlNull)
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
    pub fn parse<E, R>(
        doc_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> Result<toto_ast::GraphHandle, yaml_parser::SyntaxError>
    where
        E: AsFileEntity + From<Entity>,
        R: From<Relation> + From<FileRelation>,
    {
        let doc = ast
            .node_weight(doc_handle)
            .expect("node not found")
            .as_file()
            .expect("should be a file");

        let tree = yaml_parser::parse(doc.content.as_ref().expect("should have content"))?;
        let root = yaml_parser::ast::Root::cast(tree).expect("should be a root yaml");
        let doc = root
            .documents()
            .nth(0)
            .expect("should have at least one document");

        if let Some(b) = doc.block() {
            return Ok(Self::parse_block(b, doc_handle, ast));
        } else if let Some(f) = doc.flow() {
            return Ok(Self::parse_flow(f, doc_handle, ast));
        }

        Ok(doc_handle)
    }

    fn create_at<E, R>(
        ent: Entity,
        at: &yaml_parser::SyntaxNode,
        doc_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: From<Entity>,
        R: From<FileRelation>,
    {
        let node_handle = ast.add_node(ent.into());
        ast.add_edge(
            node_handle,
            doc_handle,
            FileRelation(usize::from(at.text_range().start())).into(),
        );

        node_handle
    }

    fn parse_flow<E, R>(
        n: yaml_parser::ast::Flow,
        doc_handle: toto_ast::GraphHandle,
        ast: &mut toto_ast::AST<E, R>,
    ) -> toto_ast::GraphHandle
    where
        E: From<Entity>,
        R: From<Relation> + From<FileRelation>,
    {
        let node_handle = Self::create_at(Entity::from(&n), n.syntax(), doc_handle, ast);
        match n.flow_map() {
            Some(m) if m.entries().is_some() => {
                for e in m.entries().unwrap().entries() {
                    let k = e.key();
                    let v = e.value();

                    let k_handle = k
                        .and_then(|k| {
                            k.flow()
                                .map(|f| Self::parse_flow(f, doc_handle, ast))
                                .or_else(|| {
                                    Some(Self::create_at(
                                        Entity::Null(YamlNull),
                                        k.syntax(),
                                        doc_handle,
                                        ast,
                                    ))
                                })
                        })
                        .or_else(|| {
                            Some(Self::create_at(
                                Entity::Null(YamlNull),
                                e.syntax(),
                                doc_handle,
                                ast,
                            ))
                        })
                        .unwrap();
                    ast.add_edge(node_handle, k_handle, Relation::from(YamlMapKey).into());

                    let v_handle = v
                        .and_then(|v| {
                            v.flow()
                                .map(|f| Self::parse_flow(f, doc_handle, ast))
                                .or_else(|| {
                                    Some(Self::create_at(
                                        Entity::Null(YamlNull),
                                        v.syntax(),
                                        doc_handle,
                                        ast,
                                    ))
                                })
                        })
                        .or_else(|| {
                            Some(Self::create_at(
                                Entity::Null(YamlNull),
                                e.syntax(),
                                doc_handle,
                                ast,
                            ))
                        })
                        .unwrap();
                    ast.add_edge(k_handle, v_handle, Relation::from(YamlMapValue).into());
                }
            }
            _ => {}
        };

        match n.flow_seq() {
            Some(s) if s.entries().is_some() => {
                for (i, v) in s.entries().unwrap().entries().enumerate() {
                    let v_handle = None
                        .or_else(|| v.flow().map(|f| Self::parse_flow(f, doc_handle, ast)))
                        .or_else(|| {
                            v.flow_pair().map(|e| {
                                let node_handle = ast.add_node(Entity::Map(YamlMap).into());
                                ast.add_edge(
                                    node_handle,
                                    doc_handle,
                                    FileRelation(usize::from(e.syntax().text_range().start()))
                                        .into(),
                                );

                                let k = e.key();
                                let v = e.value();

                                let k_handle = k
                                    .and_then(|k| {
                                        k.flow()
                                            .map(|f| Self::parse_flow(f, doc_handle, ast))
                                            .or_else(|| {
                                                Some(Self::create_at(
                                                    Entity::Null(YamlNull),
                                                    k.syntax(),
                                                    doc_handle,
                                                    ast,
                                                ))
                                            })
                                    })
                                    .or_else(|| {
                                        Some(Self::create_at(
                                            Entity::Null(YamlNull),
                                            e.syntax(),
                                            doc_handle,
                                            ast,
                                        ))
                                    })
                                    .unwrap();
                                ast.add_edge(
                                    node_handle,
                                    k_handle,
                                    Relation::from(YamlMapKey).into(),
                                );

                                let v_handle = v
                                    .and_then(|v| {
                                        v.flow()
                                            .map(|f| Self::parse_flow(f, doc_handle, ast))
                                            .or_else(|| {
                                                Some(Self::create_at(
                                                    Entity::Null(YamlNull),
                                                    v.syntax(),
                                                    doc_handle,
                                                    ast,
                                                ))
                                            })
                                    })
                                    .or_else(|| {
                                        Some(Self::create_at(
                                            Entity::Null(YamlNull),
                                            e.syntax(),
                                            doc_handle,
                                            ast,
                                        ))
                                    })
                                    .unwrap();
                                ast.add_edge(
                                    k_handle,
                                    v_handle,
                                    Relation::from(YamlMapValue).into(),
                                );

                                node_handle
                            })
                        })
                        .or_else(|| {
                            Some(Self::create_at(
                                Entity::Null(YamlNull),
                                v.syntax(),
                                doc_handle,
                                ast,
                            ))
                        })
                        .unwrap();
                    ast.add_edge(
                        node_handle,
                        v_handle,
                        Relation::from(YamlListValue(i)).into(),
                    );
                }
            }
            _ => {}
        }

        node_handle
    }

    fn parse_block<E, R>(
        n: yaml_parser::ast::Block,
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
            FileRelation(usize::from(n.syntax().text_range().start())).into(),
        );

        if let Some(m) = n.block_map() {
            for e in m.entries() {
                let k = e.key();
                let v = e.value();

                let k_handle = k
                    .and_then(|k| {
                        k.flow()
                            .map(|f| Self::parse_flow(f, doc_handle, ast))
                            .or_else(|| k.block().map(|b| Self::parse_block(b, doc_handle, ast)))
                            .or_else(|| {
                                Some(Self::create_at(
                                    Entity::Null(YamlNull),
                                    k.syntax(),
                                    doc_handle,
                                    ast,
                                ))
                            })
                    })
                    .or_else(|| {
                        Some(Self::create_at(
                            Entity::Null(YamlNull),
                            e.syntax(),
                            doc_handle,
                            ast,
                        ))
                    })
                    .unwrap();
                ast.add_edge(node_handle, k_handle, Relation::from(YamlMapKey).into());

                let v_handle = v
                    .and_then(|v| {
                        v.flow()
                            .map(|f| Self::parse_flow(f, doc_handle, ast))
                            .or_else(|| v.block().map(|b| Self::parse_block(b, doc_handle, ast)))
                            .or_else(|| {
                                Some(Self::create_at(
                                    Entity::Null(YamlNull),
                                    v.syntax(),
                                    doc_handle,
                                    ast,
                                ))
                            })
                    })
                    .or_else(|| {
                        Some(Self::create_at(
                            Entity::Null(YamlNull),
                            e.syntax(),
                            doc_handle,
                            ast,
                        ))
                    })
                    .unwrap();
                ast.add_edge(k_handle, v_handle, Relation::from(YamlMapValue).into());
            }
        }

        if let Some(s) = n.block_seq() {
            for (i, v) in s.entries().enumerate() {
                let v_handle = None
                    .or_else(|| v.flow().map(|f| Self::parse_flow(f, doc_handle, ast)))
                    .or_else(|| v.block().map(|b| Self::parse_block(b, doc_handle, ast)))
                    .or_else(|| {
                        Some(Self::create_at(
                            Entity::Null(YamlNull),
                            v.syntax(),
                            doc_handle,
                            ast,
                        ))
                    })
                    .unwrap();
                ast.add_edge(
                    node_handle,
                    v_handle,
                    Relation::from(YamlListValue(i)).into(),
                );
            }
        }

        node_handle
    }
}

pub fn as_map<E, R>(
    n: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> Option<impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)>>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    match ast.node_weight(n).expect("node not found").as_yaml() {
        Some(Entity::Map(_)) => Some(
            ast.edges(n)
                .filter_map(|e| match e.weight().as_yaml() {
                    Some(Relation::MapKey(_)) => Some(e.target()),
                    _ => None,
                })
                .map(|k| {
                    let v = ast
                        .edges(k)
                        .find_map(|e| match e.weight().as_yaml() {
                            Some(Relation::MapValue(_)) => Some(e.target()),
                            _ => None,
                        })
                        .expect("should have value by key");

                    (k, v)
                })
                .collect::<Vec<_>>()
                .into_iter(),
        ),
        _ => None,
    }
}

pub fn as_list<E, R>(
    n: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> Option<impl Iterator<Item = (usize, toto_ast::GraphHandle)>>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    match ast.node_weight(n).expect("node not found").as_yaml() {
        Some(Entity::List(_)) => Some(
            ast.edges(n)
                .filter_map(|e| match e.weight().as_yaml() {
                    Some(Relation::ListValue(i)) => Some((i.0, e.target())),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter(),
        ),
        _ => None,
    }
}

pub fn as_string<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<&YamlString>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    ast.node_weight(n)
        .expect("node not found")
        .as_yaml()
        .and_then(|yaml_node| yaml_node.try_into().ok())
}

pub fn as_int<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<&YamlInt>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    ast.node_weight(n)
        .expect("node not found")
        .as_yaml()
        .and_then(|yaml_node| yaml_node.try_into().ok())
}

pub fn as_bool<E, R>(n: toto_ast::GraphHandle, ast: &toto_ast::AST<E, R>) -> Option<&YamlBool>
where
    E: AsYamlEntity,
    R: AsYamlRelation,
{
    ast.node_weight(n)
        .expect("node not found")
        .as_yaml()
        .and_then(|yaml_node| yaml_node.try_into().ok())
}

pub fn get_lc(doc: &str, offset: usize) -> (u32, u32) {
    if offset == 0 {
        return (0, 0);
    }

    let linebreaks = doc[0..offset]
        .char_indices()
        .filter_map(|c| if c.1 == '\n' { Some(c.0) } else { None })
        .collect::<Vec<_>>();
    let lineno = linebreaks.len();
    let charno = offset - linebreaks.iter().next_back().copied().unwrap_or_default() - 1;
    (lineno as u32, charno as u32)
}

pub fn from_lc(doc: &str, lineno: u32, charno: u32) -> usize {
    let base = doc
        .split_inclusive('\n')
        .take(lineno as usize)
        .fold(0, |acc, l| acc + l.len());

    base + charno as usize
}

#[cfg(test)]
mod tests {
    extern crate derive_more;
    use derive_more::{From, TryInto};
    use petgraph::dot::Dot;

    use crate::{AsFileEntity, FileEntity, FileRelation, YamlParser};

    #[derive(Debug, From, TryInto)]
    #[try_into(owned, ref, ref_mut)]
    pub enum Entity {
        File(FileEntity),
        Yaml(crate::Entity),
    }

    #[derive(Debug, From, TryInto)]
    #[try_into(owned, ref, ref_mut)]
    pub enum Relation {
        File(FileRelation),
        Yaml(crate::Relation),
    }

    impl AsFileEntity for Entity {
        fn as_file(&self) -> Option<&FileEntity> {
            match self {
                Self::File(f) => Some(f),
                Self::Yaml(_) => None,
            }
        }
    }

    #[test]
    fn it_works() {
        let mut ast = petgraph::Graph::<Entity, Relation, petgraph::Directed, u32>::new();

        let doc_path = "file://".to_string() + env!("CARGO_MANIFEST_DIR");
        let doc_path = url::Url::parse(&doc_path).unwrap();
        let doc_path = doc_path.join("../tests/tosca_2_0.yaml").unwrap();

        let mut doc = FileEntity::from_url(doc_path);
        doc.fetch().unwrap();
        let doc_handle = ast.add_node(doc.into());

        YamlParser::parse(doc_handle, &mut ast).unwrap();
    }
}
