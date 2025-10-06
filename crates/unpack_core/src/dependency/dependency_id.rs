use crate::scheduler::COMPILER_CONTEXT;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DependencyId(u32);

impl DependencyId {
    pub fn new() -> Self {
        let dependency_id = COMPILER_CONTEXT.get().fetch_new_dependency_id();
        Self(dependency_id)
    }
}
