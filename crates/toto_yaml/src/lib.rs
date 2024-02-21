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
pub enum Entity {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List,
    Map,
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
            yaml_peg::Yaml::Null => Self::Null,
            yaml_peg::Yaml::Str(v) => Self::Str(v.to_string()),
            yaml_peg::Yaml::Int(_) => Self::Int(value.as_int().unwrap()),
            yaml_peg::Yaml::Bool(v) => Self::Bool(*v),
            yaml_peg::Yaml::Float(_) => Self::Float(value.as_float().unwrap()),
            yaml_peg::Yaml::Seq(_) => Self::List,
            yaml_peg::Yaml::Map(_) => Self::Map,
            _ => Self::Null,
        }
    }
}

pub trait AsYamlEntity {
    fn as_yaml(&self) -> Option<&Entity>;
}

pub trait AsYamlRelation {
    fn as_yaml(&self) -> Option<&Relation>;
}

pub struct Yaml(pub yaml_peg::NodeRc, pub toto_ast::GraphHandle);

impl<E, R> toto_ast::Parse<E, R> for Yaml
where
    E: From<Entity>,
    R: From<Relation> + From<FileRelation>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let self_handle = ast.add_node(Entity::from(&self.0).into());
        ast.add_edge(
            self_handle,
            self.1,
            FileRelation(self.0.pos() as usize).into(),
        );
        match self.0.yaml() {
            yaml_peg::Yaml::Map(m) => {
                for (k, v) in m.iter() {
                    let k_handle = Yaml(k.clone(), self.1).parse(ast);
                    ast.add_edge(self_handle, k_handle, Relation::MapKey.into());

                    let v_handle = Yaml(v.clone(), self.1).parse(ast);
                    ast.add_edge(k_handle, v_handle, Relation::MapValue.into());
                }
            }
            yaml_peg::Yaml::Seq(s) => {
                for (i, v) in s.iter().enumerate() {
                    let v_handle = Yaml(v.clone(), self.1).parse(ast);
                    ast.add_edge(self_handle, v_handle, Relation::ListValue(i).into());
                }
            }
            _ => {}
        }
        self_handle
    }
}

impl<E, R> toto_ast::Parse<E, R> for FileEntity
where
    E: From<FileEntity>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        ast.add_node(self.into())
    }
}

pub fn iter_keys<E: AsYamlEntity, R: AsYamlRelation>(
    root: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> impl Iterator<Item = (toto_ast::GraphHandle, toto_ast::GraphHandle)> + '_ {
    ast.edges(root)
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
}

pub fn iter_items<E: AsYamlEntity, R: AsYamlRelation>(
    root: toto_ast::GraphHandle,
    ast: &toto_ast::AST<E, R>,
) -> impl Iterator<Item = (usize, toto_ast::GraphHandle)> + '_ {
    ast.edges(root).filter_map(|e| match e.weight().as_yaml() {
        Some(Relation::ListValue(i)) => Some((*i, e.target())),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use toto_ast::Parse;

    use crate::{FileEntity, FileRelation, Yaml};

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
        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = petgraph::Graph::<Entity, Relation, petgraph::Directed, u32>::new();
        let doc_handle = FileEntity(doc.to_string()).parse(&mut ast);

        Yaml(yaml, doc_handle).parse(&mut ast);

        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
