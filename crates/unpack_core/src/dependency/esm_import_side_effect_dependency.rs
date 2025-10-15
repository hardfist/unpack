use swc_core::atoms::Atom;

use super::{AsDependencyTemplate, Dependency, DependencyId, ModuleDependency};

#[derive(Debug, Clone)]
pub struct EsmImportSideEffectDependency {
    pub request: Atom,
    pub id: DependencyId,
}

impl EsmImportSideEffectDependency {
    pub fn new(request: Atom) -> Self {
        Self {
            request,
            id: DependencyId::new(),
        }
    }
}

impl Dependency for EsmImportSideEffectDependency {
    fn id(&self) -> super::DependencyId {
        self.id
    }
}
impl ModuleDependency for EsmImportSideEffectDependency {
    fn request(&self) -> &str {
        &self.request
    }
    fn resource_identifier(&self) -> String{
        format!("{}-{}", self.context(), self.request)
    }
    fn context(&self) -> &str {
        ""
    }
}
impl AsDependencyTemplate for EsmImportSideEffectDependency {}
