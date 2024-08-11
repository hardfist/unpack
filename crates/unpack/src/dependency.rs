mod entry_dependency;
mod module_dependency;

pub use entry_dependency::{*};
pub use module_dependency::{*};
pub trait Dependency: AsModuleDependency {

}

pub type BoxDependency = Box<dyn Dependency>;