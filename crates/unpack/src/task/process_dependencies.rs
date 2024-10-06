use crate::{dependency::DependencyId, module::ModuleId};

#[derive(Debug)]
pub struct ProcessDepsTask {
    pub dependencies: Vec<DependencyId>,
    pub original_module_id: Option<ModuleId>,
}
