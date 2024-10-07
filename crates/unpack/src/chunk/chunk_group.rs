use rustc_hash::FxHashMap;
#[derive(Debug)]
pub struct ChunkGroup {
    entry_point_chunk_id: Option<ChunkId>,
    named_chunk_groups: FxHashMap<String, ChunkGroupId>,
    chunks: Vec<ChunkId>,
}
impl Default for ChunkGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkGroup {
    pub fn new() -> Self {
        Self {
            entry_point_chunk_id: None,
            chunks: vec![],
            named_chunk_groups: Default::default(),
        }
    }
    pub fn set_entry_point_chunk(&mut self, chunk_id: ChunkId) {
        self.entry_point_chunk_id = Some(chunk_id);
    }
    pub fn get_entry_point_chunk(&self) -> Option<ChunkId>{
        self.entry_point_chunk_id
    }
    pub fn connect_chunk(&mut self, chunk_id: ChunkId) {
        self.chunks.push(chunk_id);
    }
}

use index_vec::define_index_type;

use super::ChunkId;
define_index_type! {
    pub struct ChunkGroupId = u32;
}
