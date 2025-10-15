use crate::{
    dependency::{BoxDependency, DependencyId},
    module::{ModuleId, WritableModule},
};

#[derive(Debug)]
pub struct BuildTask {
    pub origin_module_id: Option<ModuleId>,
    pub module: WritableModule,
    pub dependencies: Vec<DependencyId>,
}
