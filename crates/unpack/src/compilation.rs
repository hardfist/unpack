use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    chunk::{ChunkLinker, LinkerState},
    compiler::CompilerOptions,
    errors::Diagnostics,
    module::{CodeGenerationContext, ModuleGraph, ModuleScanner, ScannerState},
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
        let module_scanner = ModuleScanner::new(self.options.clone(), self.options.context.clone());
        let mut scanner_state = ScannerState::default();
        module_scanner.add_entries(&mut scanner_state);
        scanner_state
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self, scanner_state: ScannerState) -> LinkerState {
        let mut linker_state = LinkerState::new(scanner_state.module_graph);
        let linker = ChunkLinker::new(self.options.clone(), scanner_state.entries);
        linker.build_chunk_graph(&mut linker_state);
        linker_state
    }
    /// code generation
    pub fn code_generation(&mut self, linker_state: LinkerState) {
        
        let results = linker_state.module_graph.modules.indices().collect::<Vec<_>>().into_par_iter().map(|module_id| {
            let module = linker_state.module_graph.module_by_id(module_id);
            let code_generation_result = module.code_generation(CodeGenerationContext {
                module_graph: &linker_state.module_graph
            });
            code_generation_result
        }).collect::<Vec<_>>();

    }
}
