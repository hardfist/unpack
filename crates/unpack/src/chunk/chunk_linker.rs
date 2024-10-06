use std::sync::Arc;

use indexmap::IndexMap;

use super::{chunk_graph::ChunkGraph, ChunkGroupId};
use crate::{
    compiler::CompilerOptions,
    errors::Diagnostics,
    module::{EntryData, ModuleGraph, ModuleId},
};

pub struct ChunkLinker {
    pub diagnostics: Diagnostics,
    pub options: Arc<CompilerOptions>,
    entries: IndexMap<String, EntryData>,
}
impl ChunkLinker {
    pub fn new(options: Arc<CompilerOptions>, entries: IndexMap<String, EntryData>) -> Self {
        Self {
            diagnostics: vec![],
            options,
            entries,
        }
    }
    pub fn build_chunk_graph(&self, state: &mut LinkerState) {
        let entrypoints_and_modules = self.prepare_input_entrypoints_and_modules(state);
        for (chunk_group_id, module_ids) in entrypoints_and_modules {
            //let chunk_group = state.chunk_graph.chun
        }
        //let mut visited = FxHashSet::default();
        fn visit_modules() {

        }
    }

    pub fn prepare_input_entrypoints_and_modules(
        &self,
        state: &mut LinkerState,
    ) -> IndexMap<ChunkGroupId, Vec<ModuleId>> {
        let mut entrypoint_module_map = IndexMap::default();
        for (name,entry_data) in &self.entries {
            let chunk_id = state.chunk_graph.create_chunk(Some(name.clone()));
            let chunk_group_id = state
                .chunk_graph
                .create_chunk_group(chunk_id, Some(name.clone()));
            state
                .entry_points
                .insert(name.clone(), chunk_group_id);
            let module_ids = entry_data.dependencies.iter().map(|dep_id| {
                state.module_graph.module_id_by_dependency_id(*dep_id)
            }).collect::<Vec<_>>();
            entrypoint_module_map.insert(chunk_group_id, module_ids);
        }
        entrypoint_module_map
    }
}
pub struct LinkerState {
    pub chunk_graph: ChunkGraph,
    pub module_graph: ModuleGraph,
    pub entry_points: IndexMap<String, ChunkGroupId>,
}

impl LinkerState {
    pub fn new(module_graph: ModuleGraph) -> Self {
        Self {
            chunk_graph: ChunkGraph::default(),
            entry_points: Default::default(),
            module_graph
        }
    }
}
