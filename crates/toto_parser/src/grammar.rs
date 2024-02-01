use std::path::Path;

use crate::parse::Context;

pub trait Grammar {
    fn parse<P: AsRef<Path>>(path: P, ctx: &mut Context);
    fn resolve(ctx: &mut Context);
}
