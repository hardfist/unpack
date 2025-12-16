use crate::{
    dependency::DependencyId,
    module::{ModuleId, WritableModule},
};

#[derive(Debug)]
pub struct AddModuleTask {
    pub module: WritableModule,             // to be added to module_graph
    pub module_dependency: DependencyId,    // to be added to module_graph
    pub dependencies: Vec<DependencyId>,    // recursively build
    pub origin_module_id: Option<ModuleId>, // to be added to module_graph
}
