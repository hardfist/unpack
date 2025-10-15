use crate::{
    dependency::BoxDependency,
    module::{WritableModule, ModuleId},
};

#[derive(Debug)]
pub struct AddModuleTask {
    pub module: WritableModule,                  // to be added to module_graph
    pub module_dependency: BoxDependency,   // to be added to module_graph
    pub dependencies: Vec<BoxDependency>,   // recursively build
    pub origin_module_id: Option<ModuleId>, // to be added to module_graph
}
