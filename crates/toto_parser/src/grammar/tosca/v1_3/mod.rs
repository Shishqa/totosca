use crate::grammar::Grammar;

pub mod attribute;

pub struct Tosca1_3 {}

impl Grammar for Tosca1_3 {
    fn name() -> &'static str {
        "tosca_simple_yaml_1_3"
    }
}
