use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

use super::Dependency;

pub trait ModuleDependency: Dependency + Debug + DynClone {
    fn request(&self) -> &str;
}
clone_trait_object!(ModuleDependency);

//impl_downcast!(ModuleDependency);

pub trait AsModuleDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        None
    }
    fn to_module_dependency(self: Box<Self>) -> Option<Box<dyn ModuleDependency>> {
        None
    }
}

//impl_downcast!(AsModuleDependency);
pub type BoxModuleDependency = Box<dyn ModuleDependency>;

impl<T: ModuleDependency + 'static> AsModuleDependency for T {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        Some(self)
    }
    fn to_module_dependency(self: Box<Self>) -> Option<Box<dyn ModuleDependency>> {
        Some(self)
    }
}
