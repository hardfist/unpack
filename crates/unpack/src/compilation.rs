use crate::{
    chunk::chunk_linker::{ChunkLinker, LinkerState}, compiler::CompilerOptions, errors::Diagnostics, module::{ModuleGraph, ModuleScanner, ScannerState},
};
use std::sync::Arc;

pub struct Compilation {
    #[allow(dead_code)]
    pub options: Arc<CompilerOptions>,
    module_graph: ModuleGraph,
    pub diagnostics: Diagnostics,
}

impl Compilation {
    pub fn new(options: Arc<CompilerOptions>) -> Self {
        Self {
            options,
            module_graph: Default::default(),
            diagnostics: Default::default(),
        }
    }
    /// similar with webpack's make phase, which will make module graph
    pub fn scan(&mut self) -> ScannerState {
        println!("start scan");
        let module_scanner =
            ModuleScanner::new(self.options.clone(), self.options.context.clone());
        let mut scanner_state = ScannerState::default();
        module_scanner.add_entries(&mut scanner_state);
        scanner_state
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self, scanner_state: ScannerState) -> LinkerState  {
        let mut linker_state = LinkerState::new();
        let linker = ChunkLinker::new(self.options.clone(), scanner_state.entries);
        linker.prepare_input_entrypoints_and_modules(&mut linker_state);
        linker.build_chunk_graph(&mut linker_state);
        linker_state
    }

    pub fn emit(&mut self, _linker_state: LinkerState){

    }
}
