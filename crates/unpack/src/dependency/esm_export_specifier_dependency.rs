use swc_core::atoms::Atom;

use super::{AsModuleDependency, Dependency, DependencyId, DependencyRange, DependencyTemplate};

#[derive(Debug, Clone)]
pub struct EsmExportSpecifierDependency {
    pub id: DependencyId,
    range: DependencyRange,
    name: Atom,
    value: Atom,
}

impl EsmExportSpecifierDependency {
    pub fn new(name: Atom, value: Atom, range: DependencyRange) -> Self {
        Self {
            name,
            value,
            range,
            id: DependencyId::new(),
        }
    }
}
impl Dependency for EsmExportSpecifierDependency {
    fn id(&self) -> DependencyId {
        self.id
    }
}
impl AsModuleDependency for EsmExportSpecifierDependency {}

impl DependencyTemplate for EsmExportSpecifierDependency {
    fn apply(
        &self,
        _source: &mut rspack_sources::ReplaceSource<rspack_sources::BoxSource>,
        _code_generation_context: &crate::module::CodeGenerationContext,
    ) {
    }
}
