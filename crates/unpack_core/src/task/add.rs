use crate::{
    dependency::DependencyId,
    module::{ModuleId, WritableModule},
};

#[derive(Debug)]
pub struct AddTask {
    pub module: WritableModule,
    pub module_dependency_id: DependencyId,
    pub origin_module_id: Option<ModuleId>,
}
