use swc_core::atoms::Atom;

use super::{AsDependencyTemplate, Dependency, DependencyId, ModuleDependency};

#[derive(Debug, Clone)]
pub struct HarmonyImportSideEffectDependency {
    pub request: Atom,
    pub id: DependencyId,
}

impl HarmonyImportSideEffectDependency {
    pub fn new(request: Atom) -> Self {
        Self {
            request,
            id: DependencyId::new(),
        }
    }
}

impl Dependency for HarmonyImportSideEffectDependency {
    fn id(&self) -> super::DependencyId {
        self.id
    }
}
impl ModuleDependency for HarmonyImportSideEffectDependency {
    fn request(&self) -> &str {
        &self.request
    }
}
impl AsDependencyTemplate for HarmonyImportSideEffectDependency {}
