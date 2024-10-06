use std::sync::Arc;

use indexmap::IndexMap;

use super::{chunk_graph::ChunkGraph, chunk_group::ChunkGroup, chunk_group_id::ChunkGroupId};
use crate::{compilation::Compilation, compiler::CompilerOptions, errors::Diagnostics, module::ModuleId};

pub struct ChunkLinker {
    pub diagnostics: Diagnostics,
    pub options: Arc<CompilerOptions>
}
impl ChunkLinker {
    pub fn new(options: Arc<CompilerOptions>) -> Self{
        Self{
            diagnostics: vec![],
            options
        }
    }
    pub fn build_chunk_graph(&self,state: &mut LinkerState) {
        //let mut visited = FxHashSet::default();
        fn visit_modules() {}
    }

    pub fn prepare_input_entrypoints_and_modules(&self,state: &mut LinkerState) -> IndexMap<ChunkGroupId, Vec<ModuleId>> {
        let entrypoint_module_map = IndexMap::default();
        for entry in &self.options.entry {
            let chunk_id = state.chunk_graph.create_chunk(Some(entry.name.clone()));
            let chunk_group_id = state.chunk_graph.create_chunk_group(chunk_id,Some(entry.name.clone()));
            state.entry_points.insert(entry.name.clone(), chunk_group_id);
           // entrypoint_module_map.insert(chunk_group_id, )
        }
        entrypoint_module_map
    }
}
pub struct LinkerState {
    pub chunk_graph: ChunkGraph,
    pub entry_points: IndexMap<String, ChunkGroupId>
}

impl LinkerState {
    pub fn new() -> Self{
        Self {
            chunk_graph: ChunkGraph::default(),
            entry_points: Default::default()
        }
    }


}