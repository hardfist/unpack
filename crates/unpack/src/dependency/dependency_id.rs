use index_vec::define_index_type;


use crate::module::ModuleGraph;

use super::BoxDependency;

define_index_type! {
    pub struct DependencyId = u32;
}

impl DependencyId {
    pub fn get_dependency<'a>(&self, mg: &'a ModuleGraph) -> &'a BoxDependency {
        &mg.dependencies[*self]
    }
}
