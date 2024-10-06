use index_vec::define_index_type;
use swc_core::ecma::ast::Module;

use crate::module::ModuleId;

use super::DependencyId;
#[derive(Debug,Clone)]
pub enum BlockId {
    ModuleId(ModuleId),
    DependencyId(DependencyId)
}
pub trait DependenciesBlock {
    fn add_block_id(&mut self, block_id: BlockId);
    fn get_blocks(&self) -> Vec<BlockId>;
    fn add_dependency_id(&mut self, dependency_id: DependencyId);
    fn get_dependencies(&self) -> Vec<DependencyId>;
}

pub struct AsyncDependenciesBlock {
    dependencies: Vec<DependencyId>
}
