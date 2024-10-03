use index_vec::IndexVec;

use crate::{
    dependency::{BoxDependency, DependencyId},
    module::{BoxModule, ModuleId},
};

#[derive(Debug, Default)]
pub struct ModuleGraph {
    pub dependencies: IndexVec<DependencyId, BoxDependency>,
    pub modules: IndexVec<ModuleId, BoxModule>,
}

impl ModuleGraph {
    pub fn add_dependency(&mut self, dep: BoxDependency) -> DependencyId {
        self.dependencies.push(dep)
    }
    pub fn add_module(&mut self, module: BoxModule) -> ModuleId {
        self.modules.push(module)
    }
    // get dependency by id
    pub fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        &self.dependencies[id]
    }
     pub fn dependency_by_id_mut(&mut self, id: DependencyId) -> &mut BoxDependency {
        &mut self.dependencies[id]
    }
    pub fn module_by_id(&self, id: ModuleId) -> &BoxModule {
        &self.modules[id]
    }
    pub fn module_by_id_mut(&mut self, id: ModuleId) -> &mut BoxModule {
        &mut self.modules[id]
    }
}
