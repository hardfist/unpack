use crate::{
    dependency::DependencyId,
    module::{WritableModule, ModuleId},
};

#[derive(Debug)]
pub struct AddTask {
    pub module: WritableModule,
    pub module_dependency_id: DependencyId,
    pub origin_module_id: Option<ModuleId>,
}
