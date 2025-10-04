use index_vec::define_index_type;
use indexmap::{IndexMap, IndexSet};

use crate::module::ModuleId;

use super::ChunkId;

#[derive(Debug)]
pub struct ChunkGraphChunk {
    pub entry_modules: IndexMap<ModuleId, ChunkId>,
    pub modules: IndexSet<ModuleId>,
    pub runtime_modules: IndexSet<ModuleId>,
}
impl Default for ChunkGraphChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkGraphChunk {
    pub fn new() -> Self {
        Self {
            entry_modules: Default::default(),
            modules: Default::default(),
            runtime_modules: Default::default(),
        }
    }
}

define_index_type! {
    pub struct ChunkGraphChunkId = u32;
}
