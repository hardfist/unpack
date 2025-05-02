use crate::dependency::{BoxDependency, DependencyId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_dependency(&mut self, dep: BoxDependency) -> DependencyId {
        let dep_id = dep.id();
        self.dependencies.insert(dep_id, dep);
        dep_id
    }
    // get dependency by id
    pub fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        self.dependencies.get(&id).expect("get dependency failed")
    }
    pub fn dependency_by_id_mut(&mut self, id: DependencyId) -> &mut BoxDependency {
        self.dependencies
            .get_mut(&id)
            .expect("get depependency failed")
    }
}
