use std::sync::atomic::AtomicU32;

use crate::module::ModuleGraph;

use super::BoxDependency;

pub static DEPENDENCY_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DependencyId(u32);

impl DependencyId {
    pub fn get_dependency<'a>(&self, mg: &'a ModuleGraph) -> &'a BoxDependency {
        &mg.dependencies.get(self).expect("get dependency failed")
    }
}

impl DependencyId {
    pub fn new() -> Self {
        Self(DEPENDENCY_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}
