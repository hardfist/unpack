use crate::{
    dependency::BoxDependency,
    module::{BoxModule, ModuleId},
};

#[derive(Debug)]
pub struct BuildTask {
    pub origin_module_id: Option<ModuleId>,
    pub module: BoxModule,
    pub module_dependency: BoxDependency,
}
