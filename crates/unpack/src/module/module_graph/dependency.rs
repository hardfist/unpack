use crate::dependency::{BoxDependency, DependencyId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_dependency(&mut self, dep: BoxDependency) -> DependencyId {
        self.dependencies.push(dep)
    }
    // get dependency by id
    pub fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        &self.dependencies[id]
    }
    pub fn dependency_by_id_mut(&mut self, id: DependencyId) -> &mut BoxDependency {
        &mut self.dependencies[id]
    }
}
