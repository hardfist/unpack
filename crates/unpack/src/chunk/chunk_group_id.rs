use index_vec::define_index_type;
use index_vec::IndexVec;

use super::chunk_group::ChunkGroup;
define_index_type! {
    pub struct ChunkGroupId = u32;
}

pub type ChunkGroupVec = IndexVec<ChunkGroupId, ChunkGroup>;
