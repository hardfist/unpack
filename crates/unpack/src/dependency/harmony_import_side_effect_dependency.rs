use swc_core::atoms::Atom;

use super::{AsDependencyTemplate, AsModuleDependency, Dependency, ModuleDependency};

#[derive(Debug, Clone)]
pub struct HarmonyImportSideEffectDependency {
    pub request: Atom,
}

impl Dependency for HarmonyImportSideEffectDependency {}
impl ModuleDependency for HarmonyImportSideEffectDependency {
    fn request(&self) -> &str {
        &self.request
    }
}
impl AsDependencyTemplate for HarmonyImportSideEffectDependency {
    
}