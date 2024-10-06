use index_vec::IndexVec;
use rustc_hash::FxHashMap;

use super::{chunk_id::ChunkId, Chunk};

#[derive(Debug,Default)]
pub struct ChunkGraph {
    named_chunks: FxHashMap<String, ChunkId>,
    chunks: IndexVec<ChunkId, Chunk>
}

impl ChunkGraph {
    fn create_chunk(&mut self,name:Option<String>){
        let chunk = Chunk::new(name.clone());
        let chunk_id = self.add_chunk(chunk);
        if let Some(name) = name {
            self.named_chunks.insert(name, chunk_id);
        }
        
    }
    fn add_chunk(&mut self, chunk:Chunk)-> ChunkId {
        self.chunks.push(chunk)
    }
}