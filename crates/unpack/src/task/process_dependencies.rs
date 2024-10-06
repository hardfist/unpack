use crate::{dependency::DependencyId, module::ModuleId};

#[derive(Debug)]
pub(crate) struct ProcessDepsTask {
    pub(crate)dependencies: Vec<DependencyId>,
    pub(crate) original_module_id: Option<ModuleId>
}