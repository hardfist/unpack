use index_vec::IndexVec;
use index_vec::define_index_type;
use super::NormalModule;
define_index_type! {
    pub struct ModuleId = u32;
}

pub type ModuleVec = IndexVec<ModuleId, NormalModule>;

