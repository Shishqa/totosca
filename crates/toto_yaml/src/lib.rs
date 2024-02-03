use std::fmt::Debug;

pub struct Entity(yaml_peg::NodeRc);

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.yaml() {
            yaml_peg::Yaml::Map(_) => f.write_str("Map"),
            yaml_peg::Yaml::Seq(_) => f.write_str("List"),
            _ => self.0.fmt(f),
        }
    }
}

#[derive(Debug)]
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

impl toto_ast::Entity for Entity {}
impl toto_ast::Relation for Relation {}

impl toto_ast::ToAST for Entity {
    fn to_ast(self, ast: &mut toto_ast::AST) -> toto_ast::GraphHandle {
        let self_handle = ast.add_node(Box::new(Entity(self.0.clone())));
        match self.0.yaml() {
            yaml_peg::Yaml::Map(m) => {
                for (k, v) in m.iter() {
                    let k_handle = Entity::from(k.clone()).to_ast(ast);
                    ast.add_edge(self_handle, k_handle, Box::new(Relation::MapKey));

                    let v_handle = Entity::from(v.clone()).to_ast(ast);
                    ast.add_edge(k_handle, v_handle, Box::new(Relation::MapValue));
                }
            }
            yaml_peg::Yaml::Seq(s) => {
                for (i, v) in s.iter().enumerate() {
                    let v_handle = Entity::from(v.clone()).to_ast(ast);
                    ast.add_edge(self_handle, v_handle, Box::new(Relation::ListValue(i)));
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
    use toto_ast::ToAST;

    use crate::Entity;

    #[test]
    fn it_works() {
        let doc = include_str!("../../../tests/a.yaml");

        let yaml = yaml_peg::parse::<yaml_peg::repr::RcRepr>(doc)
            .unwrap()
            .remove(0);

        let mut ast = toto_ast::AST::new();

        Entity::from(yaml.clone()).to_ast(&mut ast);

        dbg!(Dot::new(&ast));
    }
}
