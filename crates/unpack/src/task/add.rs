use crate::{dependency::DependencyId, module::ModuleId};

#[derive(Debug)]
pub struct AddTask {
    pub module_id: ModuleId,
    pub module_dependency_id: DependencyId,
    pub origin_module_id: Option<ModuleId>,
}
