use crate::dependency::{BoxDependency};
use crate::module::ModuleId;
use crate::task::Task;
#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency: BoxDependency,
    pub(crate) origin_module_id: Option<ModuleId>
}

impl Task for FactorizeTask {
    fn run(&self) -> super::TaskResult {
        Ok(vec![])
    }
}