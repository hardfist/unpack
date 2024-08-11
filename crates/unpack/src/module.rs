mod module_id;
pub use module_id::*;
pub trait Module {}

pub type BoxModule = Box<dyn Module>;
pub struct NormalModule {}
