use swc_core::atoms::Atom;

use super::{AsModuleDependency, Dependency, ModuleDependency};

#[derive(Debug,Clone)]
pub struct HarmonyImportSideEffectDependency {
    pub request: Atom,
}

impl Dependency for HarmonyImportSideEffectDependency {
  
}
impl ModuleDependency for HarmonyImportSideEffectDependency {
    fn request(&self) -> &str {
        &self.request
    }
}
impl AsModuleDependency for HarmonyImportSideEffectDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        Some(self)
    }
    fn into_module_dependency(self: Box<Self>) -> Option<Box<dyn ModuleDependency>> {
        Some(self)
    }
}