mod module_id;
use std::fmt::Debug;

pub use module_id::*;
pub trait Module: Debug {}

pub type BoxModule = Box<dyn Module>;
pub struct NormalModule {}
