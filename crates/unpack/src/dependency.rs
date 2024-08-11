mod entry_dependency;
mod module_dependency;
mod dependency_id;

pub use entry_dependency::*;
pub use module_dependency::*;
pub use dependency_id::*;
pub trait Dependency: AsModuleDependency {}

pub type BoxDependency = Box<dyn Dependency>;
