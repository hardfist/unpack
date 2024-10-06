use rustc_hash::FxHashMap;

use super::{chunk_group_id::ChunkGroupId, chunk_id::ChunkId};

#[derive(Debug)]
pub struct ChunkGroup {
    entry_point_chunk_id: Option<ChunkId>,
    named_chunk_groups: FxHashMap<String, ChunkGroupId>,
    chunks: Vec<ChunkId>,
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
    pub fn connect_chunk(&mut self, chunk_id: ChunkId) {
        self.chunks.push(chunk_id);
    }
}
