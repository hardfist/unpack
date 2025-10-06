use crate::{
    dependency::BoxDependency,
    module::{BoxModule, ModuleId},
};

#[derive(Debug)]
pub struct AddModuleTask {
    pub module: BoxModule,                  // to be added to module_graph
    pub module_dependency: BoxDependency,   // to be added to module_graph
    pub dependencies: Vec<BoxDependency>,   // recursively build
    pub origin_module_id: Option<ModuleId>, // to be added to module_graph
}
