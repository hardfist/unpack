mod entry_dependency;
mod module_dependency;
mod dependency_id;

use std::fmt::Debug;

pub use entry_dependency::*;
pub use module_dependency::*;
pub use dependency_id::*;


pub trait Dependency: AsModuleDependency + Debug {}

pub type BoxDependency = Box<dyn Dependency>;
