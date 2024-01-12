use crate::parse::Context;

pub trait Grammar {
    fn parse(doc: &str, ctx: &mut Context);
}
