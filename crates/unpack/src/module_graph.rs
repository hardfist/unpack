use index_vec::IndexVec;

use crate::{
    dependency::{BoxDependency, DependencyId},
    module::{BoxModule, ModuleId},
};

#[derive(Debug, Default)]
pub struct ModuleGraph {
    pub(crate) dependencies: IndexVec<DependencyId, BoxDependency>,
    pub(crate) modules: IndexVec<ModuleId, BoxModule>,
}

impl ModuleGraph {
    pub(crate) fn add_dependency(&mut self, dep: BoxDependency) -> DependencyId {
        self.dependencies.push(dep)
    }
    pub(crate) fn add_module(&mut self, module: BoxModule) -> ModuleId {
        self.modules.push(module)
    }
    // get dependency by id
    pub(crate) fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        &self.dependencies[id]
    }
     pub(crate) fn dependency_by_id_mut(&mut self, id: DependencyId) -> &mut BoxDependency {
        &mut self.dependencies[id]
    }
    pub(crate) fn module_by_id(&self, id: ModuleId) -> &BoxModule {
        &self.modules[id]
    }
    pub(crate) fn module_by_id_mut(&mut self, id: ModuleId) -> &mut BoxModule {
        &mut self.modules[id]
    }
}
