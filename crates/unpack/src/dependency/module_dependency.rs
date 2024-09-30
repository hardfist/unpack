use std::fmt::Debug;

use dyn_clone::clone_trait_object;

use super::Dependency;

pub trait ModuleDependency: Dependency + Debug {
    fn request(&self) -> &str;
}
clone_trait_object!(ModuleDependency);

//impl_downcast!(ModuleDependency);

pub trait AsModuleDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        None
    }
}

//impl_downcast!(AsModuleDependency);
pub type BoxModuleDependency = Box<dyn ModuleDependency>;
