use index_vec::IndexVec;
use rustc_hash::FxHashMap;

use crate::module::ModuleId;

use super::{chunk_graph_chunk, chunk_graph_module, chunk_group::ChunkGroup, Chunk, ChunkGraphChunk, ChunkGraphChunkId, ChunkGraphModule, ChunkGraphModuleId, ChunkGroupId, ChunkId};

#[derive(Debug, Default)]
pub struct ChunkGraph {
    named_chunks: FxHashMap<String, ChunkId>,
    named_chunk_groups: FxHashMap<String, ChunkGroupId>,
    chunks: IndexVec<ChunkId, Chunk>,
    chunk_graph_chunks: IndexVec<ChunkGraphChunkId, ChunkGraphChunk>,
    chunk_graph_modules: IndexVec<ChunkGraphModuleId, ChunkGraphModule>,
    chunk_id_to_chunk_graph_chunk_id: FxHashMap<ChunkId, ChunkGraphChunkId>,
    module_id_to_chunk_graph_module_id: FxHashMap<ModuleId, ChunkGraphModuleId>,
    chunk_groups: IndexVec<ChunkGroupId, ChunkGroup>,
}

impl ChunkGraph {
    pub fn create_chunk(&mut self, name: Option<String>) -> ChunkId {
        let chunk = Chunk::new(name.clone());
        let chunk_id = self.add_chunk(chunk);
        if let Some(name) = name {
            self.named_chunks.insert(name, chunk_id);
        }
        chunk_id
    }
    pub fn create_chunk_group(
        &mut self,
        entry_chunk_id: ChunkId,
        name: Option<String>,
    ) -> ChunkGroupId {
        let mut chunk_group = ChunkGroup::new();
        chunk_group.connect_chunk(entry_chunk_id);
        let chunk_group_id = self.add_chunk_group(chunk_group);
        if let Some(name) = name {
            self.named_chunk_groups.insert(name, chunk_group_id);
        }
        chunk_group_id
    }
    pub fn add_chunk(&mut self, chunk: Chunk) -> ChunkId {
        self.chunks.push(chunk)
    }
    pub fn add_chunk_group(&mut self, chunk_group: ChunkGroup) -> ChunkGroupId {
        self.chunk_groups.push(chunk_group)
    }
    pub fn chunk_by_id(&self, chunk_id: ChunkId) -> &Chunk {
        &self.chunks[chunk_id]
    }
    pub fn chunk_group_by_id(&self, chunk_group_id: ChunkGroupId) -> &ChunkGroup {
        &self.chunk_groups[chunk_group_id]
    }
    pub fn chunk_group_by_id_mut(&mut self, chunk_group_id: ChunkGroupId) -> &mut ChunkGroup {
        &mut self.chunk_groups[chunk_group_id]
    }
    pub fn connect_chunk_and_entry_module(&mut self, chunk_id: ChunkId, module_id: ModuleId, entry_point_id: ChunkGroupId){
        
    }
    pub fn chunk_graph_chunk_by_id(&self, chunk_graph_chunk_id: ChunkGraphChunkId) -> &ChunkGraphChunk {
        &self.chunk_graph_chunks[chunk_graph_chunk_id]
    }
    pub fn chunk_graph_chunk_by_id_mut(&mut self,chunk_graph_chunk_id: ChunkGraphChunkId) -> &mut ChunkGraphChunk {
        &mut self.chunk_graph_chunks[chunk_graph_chunk_id]
    }
    pub fn chunk_graph_chunk_id_by_chunk_id(&mut self, chunk_id: ChunkId) ->ChunkGraphChunkId{
        let chunk_graph_chunk_id = if let Some(id) = self.chunk_id_to_chunk_graph_chunk_id.get(&chunk_id) {
            *id
        }else {
            let chunk_graph_chunk = ChunkGraphChunk::new();
            let chunk_graph_chunk_id = self.chunk_graph_chunks.push(chunk_graph_chunk);
            chunk_graph_chunk_id
        };
        chunk_graph_chunk_id
    }
    pub fn is_module_in_chunk(&mut self, module_id: ModuleId, chunk_id: ChunkId) -> bool {
        let cgc_id = self.chunk_graph_chunk_id_by_chunk_id(chunk_id);
        let chunk_graph_chunk = self.chunk_graph_chunk_by_id(cgc_id);
        chunk_graph_chunk.modules.contains(&module_id)
    }
    pub fn chunk_graph_module_id_by_module_id(&mut self,module_id: ModuleId)-> ChunkGraphModuleId{
        let chunk_graph_module_id = if let Some(id) =  self.module_id_to_chunk_graph_module_id.get(&module_id){
            *id
        }else {
            let chunk_graph_module = ChunkGraphModule::new();
            let chunk_graph_module_id = self.chunk_graph_modules.push(chunk_graph_module);
            chunk_graph_module_id
        };
        chunk_graph_module_id
    }
    pub fn chunk_graph_module_by_id(&self, chunk_graph_module_id:ChunkGraphModuleId) -> &ChunkGraphModule {
        &self.chunk_graph_modules[chunk_graph_module_id]
    }
    pub fn chunk_graph_module_by_id_mut(&mut self, chunk_graph_module_id:ChunkGraphModuleId) -> &mut ChunkGraphModule {
        &mut self.chunk_graph_modules[chunk_graph_module_id]
    }
    // connect chunk & module by chunk_graph_module
    pub fn connect_chunk_and_module(&mut self,chunk_id: ChunkId, module_id: ModuleId){
        // connect chunk with module
        let chunk_graph_module_id = self.chunk_graph_module_id_by_module_id(module_id);
        let chunk_graph_module = self.chunk_graph_module_by_id_mut(chunk_graph_module_id);
        chunk_graph_module.chunks.insert(chunk_id);

        // connect module with chunk
        let chunk_graph_chunk_id = self.chunk_graph_chunk_id_by_chunk_id(chunk_id);
        let chunk_graph_chunk  = self.chunk_graph_chunk_by_id_mut(chunk_graph_chunk_id);
        chunk_graph_chunk.modules.insert(module_id);
        
    }
}
