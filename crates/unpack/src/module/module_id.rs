use super::NormalModule;
use index_vec::define_index_type;
use index_vec::IndexVec;
define_index_type! {
    pub struct ModuleId = u32;
}

pub type ModuleVec = IndexVec<ModuleId, NormalModule>;
