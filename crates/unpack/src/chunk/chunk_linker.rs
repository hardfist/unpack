use super::chunk_graph::ChunkGraph;
use crate::{compilation::Compilation, errors::Diagnostics};

pub struct ChunkLinker {
    pub diagnostics: Diagnostics
}
impl ChunkLinker {
    pub fn new() -> Self{
        Self{
            diagnostics: vec![]
        }
    }
    pub fn build_chunk_graph(state: &mut LinkerState) {
        //let mut visited = FxHashSet::default();
        fn visit_modules() {}
    }

    pub fn prepare_input_entrypoints_and_modules(compilation: &Compilation, state: &mut LinkerState) {
        for entry in &compilation.options.entry {}
    }
}
pub struct LinkerState {
    pub chunk_graph: ChunkGraph
}

impl LinkerState {
    pub fn new() -> Self{
        Self {
            chunk_graph: ChunkGraph::default()
        }
    }
}