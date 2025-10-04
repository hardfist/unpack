use index_vec::define_index_type;
use indexmap::IndexSet;

use super::ChunkId;

#[derive(Debug, Clone, Default)]
pub struct ChunkGraphModule {
    pub entry_in_chunks: IndexSet<ChunkId>,
    pub chunks: IndexSet<ChunkId>,
    pub runtime_in_chunks: IndexSet<ChunkId>,
}
impl ChunkGraphModule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
define_index_type! {
    pub struct ChunkGraphModuleId = u32;
}
