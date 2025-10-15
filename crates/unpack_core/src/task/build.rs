use crate::{
    dependency::BoxDependency,
    module::{WritableModule, ModuleId},
};

#[derive(Debug)]
pub struct BuildTask {
    pub origin_module_id: Option<ModuleId>,
    pub module: WritableModule,
    pub dependencies: Vec<BoxDependency>,
}
