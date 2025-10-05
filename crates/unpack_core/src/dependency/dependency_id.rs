
use crate::{module::ModuleGraph, scheduler::COMPILER_CONTEXT};

use super::BoxDependency;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DependencyId(u32);

impl DependencyId {
    pub fn get_dependency<'a>(&self, mg: &'a ModuleGraph) -> &'a BoxDependency {
        mg.dependencies.get(self).expect("get dependency failed")
    }
}

impl DependencyId {
    pub fn new() -> Self {
        let dependency_id = COMPILER_CONTEXT.get().fetch_new_dependency_id();
        Self(dependency_id)
    }
}
