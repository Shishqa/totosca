use petgraph::Graph;

pub trait Entity {}

pub trait Edge {}

pub type IR<'ir> = Graph<&'ir dyn Entity, &'ir dyn Edge>;
