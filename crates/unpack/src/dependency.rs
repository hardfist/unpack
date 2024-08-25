mod entry_dependency;
mod module_dependency;
mod dependency_id;

use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};
pub use entry_dependency::*;
pub use module_dependency::*;
pub use dependency_id::*;


pub trait Dependency: AsModuleDependency + Debug + DynClone {}

clone_trait_object!(Dependency);

pub type BoxDependency = Box<dyn Dependency>;
