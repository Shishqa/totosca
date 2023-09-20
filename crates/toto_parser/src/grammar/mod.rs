pub mod tosca;

pub trait Grammar {
    fn parse(doc: &str);
}
