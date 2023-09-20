pub mod attribute;
pub mod collection;
pub mod data;
pub mod file;
pub mod import;
pub mod list;
pub mod map;
pub mod node;
pub mod parameter;
pub mod property;
pub mod requirement;
pub mod schema;
pub mod service_template;
pub mod value;

pub use attribute::*;
pub use collection::*;
pub use data::*;
pub use file::*;
pub use import::*;
pub use list::*;
pub use map::*;
pub use node::*;
pub use parameter::*;
pub use property::*;
pub use requirement::*;
pub use schema::*;
pub use service_template::*;

// pub struct Tosca2_0 {}
//
// impl Grammar for Tosca2_0 {
//     fn name() -> &'static str {
//         "tosca_2_0"
//     }
// }
