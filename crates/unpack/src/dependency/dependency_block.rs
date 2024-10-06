use index_vec::define_index_type;

use crate::module::ModuleId;

use super::DependencyId;
pub trait DependenciesBlock {
    fn add_block_id(&mut self, block_id: AsyncDependenciesBlockId);
    fn get_blocks(&self) -> Vec<AsyncDependenciesBlockId>;
    fn add_dependency_id(&mut self, dependency_id: DependencyId);
    fn get_dependencies(&self) -> Vec<DependencyId>;
}

pub struct AsyncDependenciesBlock {
    dependencies: Vec<DependencyId>
}
define_index_type! {
    pub struct AsyncDependenciesBlockId = u32;
}

#[derive(Debug,Clone,Copy)]
pub enum BlockId {
    ModuleId(ModuleId),
    AsyncDependenciesBlockId(AsyncDependenciesBlockId)
}