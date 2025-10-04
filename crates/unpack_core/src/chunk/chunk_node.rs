use index_vec::define_index_type;
use index_vec::IndexVec;
#[derive(Debug)]
pub struct Chunk {
    pub name: Option<String>,
}

impl Chunk {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }
}

define_index_type! {
    pub struct ChunkId = u32;
}

pub type ChunkVec = IndexVec<ChunkId, Chunk>;
