use crate::{
    dependency::DependencyId,
    module::{BoxModule, ModuleId},
};

#[derive(Debug)]
pub struct AddTask {
    pub module: BoxModule,
    pub module_dependency_id: DependencyId,
    pub origin_module_id: Option<ModuleId>,
}
