use std::{collections::VecDeque, sync::Arc};

use indexmap::IndexMap;

use super::{chunk_graph::ChunkGraph, ChunkGroupId, ChunkId};
use crate::{
    compiler::CompilerOptions, dependency::BlockId, errors::Diagnostics, module::{EntryData, ModuleGraph, ModuleId}
};
enum QueueAction {
   AddAndEnterEntryModule(AddAndEnterEntryModule),
   AddAndEnterModule(AddAndEnterModule),
   LeaveModule(LeaveModule),
   ProcessBlock(ProcessBlock)
}
struct ProcessBlock {
    module_id: ModuleId,
    block_id: BlockId,
    chunk: ChunkId
}
struct AddAndEnterEntryModule {

}
struct AddAndEnterModule {
    module_id: ModuleId,
    chunk_id: ChunkId
}
struct LeaveModule {

}
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
            entries
        }
    }
    pub fn build_chunk_graph(&self, state: &mut LinkerState) {
        let entrypoints_and_modules = self.prepare_input_entrypoints_and_modules(state);
        for (chunk_group_id, module_ids) in entrypoints_and_modules {
            let chunk_group = state.chunk_graph.chunk_group_by_id(chunk_group_id);
            let entry_point_chunk_id = chunk_group.get_entry_point_chunk().expect("should get entry_chunk");
            for module_id in module_ids {
                state.queue.push_back(
                    QueueAction::AddAndEnterModule(AddAndEnterModule{
                        module_id,
                        chunk_id:entry_point_chunk_id
                    })
                )
            }
        }
        while let Some(action) = state.queue.pop_front() {
            self.handle_queue_action(state, action);
        }
    }
    pub fn handle_queue_action(&self,state:&mut LinkerState,action: QueueAction){
        match action {
            QueueAction::AddAndEnterEntryModule(action) => {
                self.add_and_enter_entry_module(state, action);
            },
            QueueAction::AddAndEnterModule(action) => {
                self.add_and_enter_module(state, action);
            }
            _ => {
                todo!("no implemented yet")
            }
        }
    }
    pub fn add_and_enter_entry_module(&self,state:&mut LinkerState,  action: AddAndEnterEntryModule){

    }
    pub fn add_and_enter_module(&self, state: &mut LinkerState, action: AddAndEnterModule) {

    }
    pub fn process_block(&mut self,state: &mut LinkerState, action: ProcessBlock){

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
            let chunk_group = state.chunk_graph.chunk_group_by_id_mut(chunk_group_id);
            chunk_group.set_entry_point_chunk(chunk_id);
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
    pub queue: VecDeque<QueueAction>
}

impl LinkerState {
    pub fn new(module_graph: ModuleGraph) -> Self {
        Self {
            chunk_graph: ChunkGraph::default(),
            entry_points: Default::default(),
            module_graph,
            queue: Default::default()
        }
    }
}
