use index_vec::define_index_type;
use index_vec::IndexVec;

use super::Chunk;
define_index_type! {
    pub struct ChunkId = u32;
}

pub type ModuleVec = IndexVec<ChunkId, Chunk>;
