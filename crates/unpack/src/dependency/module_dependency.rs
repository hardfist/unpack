use super::Dependency;

pub trait ModuleDependency: Dependency {}
pub trait AsModuleDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        None
    }
}
