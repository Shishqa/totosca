use std::fmt::Debug;

// TODO: this can be stored in a parsed way here
#[derive(Clone)]
pub struct Entity(pub yaml_peg::NodeRc);

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.yaml() {
            yaml_peg::Yaml::Map(_) => f.write_str("Map"),
            yaml_peg::Yaml::Seq(_) => f.write_str("List"),
            _ => self.0.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Relation {
    MapKey,
    MapValue,
    ListValue(usize),
}

impl From<yaml_peg::NodeRc> for Entity {
    fn from(value: yaml_peg::NodeRc) -> Self {
        Self(value)
    }
}

pub trait AsYamlEntity {
    fn as_yaml(&self) -> Option<&Entity>;
}

pub trait AsYamlRelation {
    fn as_yaml(&self) -> Option<&Relation>;
}

impl<E, R> toto_ast::Parse<E, R> for Entity
where
    E: From<Entity>,
    R: From<Relation>,
{
    fn parse(self, ast: &mut toto_ast::AST<E, R>) -> toto_ast::GraphHandle {
        let self_handle = ast.add_node(Entity(self.0.clone()).into());
        match self.0.yaml() {
            yaml_peg::Yaml::Map(m) => {
                for (k, v) in m.iter() {
                    let k_handle = Entity::from(k.clone()).parse(ast);
                    ast.add_edge(self_handle, k_handle, Relation::MapKey.into());

                    let v_handle = Entity::from(v.clone()).parse(ast);
                    ast.add_edge(k_handle, v_handle, Relation::MapValue.into());
                }
            }
            yaml_peg::Yaml::Seq(s) => {
                for (i, v) in s.iter().enumerate() {
                    let v_handle = Entity::from(v.clone()).parse(ast);
                    ast.add_edge(self_handle, v_handle, Relation::ListValue(i).into());
                }
            }
            _ => {}
        }
        self_handle
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use toto_ast::Parse;

    use crate::{Entity, Relation};

    #[test]
    fn it_works() {
        let doc = include_str!("../../../tests/a.yaml");

        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = petgraph::Graph::<Entity, Relation, petgraph::Directed, u32>::new();

        Entity::from(yaml.clone()).parse(&mut ast);

        dbg!(Dot::new(&ast));

        assert!(false);
    }
}
