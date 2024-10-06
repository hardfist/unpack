use indexmap::{IndexMap, IndexSet};

use crate::module::ModuleId;

use super::ChunkId;

pub struct ChunkGraphChunk {
    pub entry_modules: IndexMap<ModuleId, ChunkId>,
    pub modules: IndexSet<ModuleId>,
    pub runtime_modules: IndexSet<ModuleId>
}
impl ChunkGraphChunk {
    pub fn new() -> Self {
        Self {
            entry_modules: Default::default(),
            modules: Default::default(),
            runtime_modules: Default::default()
        }
    }
}