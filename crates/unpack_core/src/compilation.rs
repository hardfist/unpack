use indexmap::IndexSet;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rspack_sources::{BoxSource, ConcatSource, SourceExt};
use rustc_hash::FxHashMap;

use crate::{
    chunk::{ChunkGraph, ChunkId, ChunkLinker, LinkerResult}, compiler::CompilerOptions, errors::Diagnostics, memory_manager::MemoryManager, module::{
        self, CodeGenerationContext, CodeGenerationResult, ModuleGraph, ModuleId, ModuleScanner, ScannerResult
    }, plugin::PluginDriver
};
use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Instant,
};
#[derive(Debug, Default)]
struct CodeGenerationResults {
    module_id_to_generation_result: FxHashMap<ModuleId, CodeGenerationResult>,
}
pub struct CodeGenerationState {
    chunk_graph: ChunkGraph,
    code_generation_results: CodeGenerationResults,
    pub diagnostics: Diagnostics,
}
#[derive(Debug, Clone)]
pub struct ChunkAssetState {
    pub assets: FxHashMap<String, BoxSource>,
}

static COMPILATION_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompilationId(pub u32);

impl CompilationId {
    pub fn new() -> Self {
        Self(COMPILATION_ID.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Debug)]
pub struct Compilation {
    pub id: CompilationId,
    #[allow(dead_code)]
    pub options: Arc<CompilerOptions>,
    pub module_graph: ModuleGraph,
    pub diagnostics: Diagnostics,
    pub plugin_driver: Arc<PluginDriver>,
}
impl Drop for Compilation {
    fn drop(&mut self) {
        println!("compilation:{} drop", self.id.0);
    }
}

impl Compilation {
    pub fn new(options: Arc<CompilerOptions>, plugin_driver: Arc<PluginDriver>) -> Self {
        let id = CompilationId::new();
        println!("create compilation: {:?}", &id);
        Self {
            options,
            module_graph: Default::default(),
            diagnostics: Default::default(),
            plugin_driver,
            id,
        }
    }
    /// similar with webpack's make phase, which will make module graph
    pub async fn scan<'a>(&self,memory_manager: &'a mut MemoryManager) -> ScannerResult {
        let start = Instant::now();
        let mut module_scanner = ModuleScanner::new(
            self.options.clone(),
            self.options.context.clone(),
            self.plugin_driver.clone(),
        );
        
        let scanner_result = module_scanner.from_entries(memory_manager).await;
        
        let elapsed = start.elapsed();
        println!(
            "scan finished with {} modules in {:?}",
            scanner_result._modules.len(),
            elapsed
        );
        scanner_result
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&self, scanner_state: ScannerResult) -> LinkerResult {
        let mut linker_state =
            LinkerResult::new(scanner_state.module_graph, scanner_state.diagnostics);
        let linker = ChunkLinker::new(self.options.clone(), scanner_state.entries);
        linker.build_chunk_graph(&mut linker_state);
        linker_state
    }
    /// code generation
    pub fn code_generation(&self, linker_state: LinkerResult,memory_manager: &mut MemoryManager) -> CodeGenerationState {
        let mut code_generation_results = CodeGenerationResults::default();
        let results = linker_state
            .module_graph
            .modules
            .clone()
            .into_par_iter()
            .map(|module_id| {
                let module = memory_manager.module_by_id(module_id);
                let codegen_result = module.code_generation(CodeGenerationContext {
                    module_graph: &linker_state.module_graph,
                });
                (module_id, codegen_result)
            })
            .collect::<Vec<_>>();
        for (id, result) in results {
            // FIXME: fixed codegeneration diagnostics later
            code_generation_results
                .module_id_to_generation_result
                .insert(id, result.expect("codegeneration failed"));
        }
        CodeGenerationState {
            chunk_graph: linker_state.chunk_graph,
            code_generation_results,
            diagnostics: linker_state.diagnostics,
        }
    }
    // chunk asset
    pub fn create_chunk_asset(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> ChunkAssetState {
        // let manifest = vec![];
        let mut assets = FxHashMap::default();
        for chunk_id in code_generation_state.chunk_graph.chunks.indices() {
            let chunk_name = code_generation_state
                .chunk_graph
                .chunk_by_id(chunk_id)
                .name
                .to_owned()
                .expect("should have name");
            let chunk_modules = code_generation_state
                .chunk_graph
                .get_chunk_modules(chunk_id);
            let chunk_source =
                self.render_chunk_modules(code_generation_state, chunk_id, chunk_modules);
            let chunk_name = format!("{}{}", chunk_name, ".js");
            assets.insert(chunk_name, chunk_source);
        }
        ChunkAssetState { assets }
    }
    pub fn render_chunk_modules(
        &self,
        state: &mut CodeGenerationState,
        chunk_id: ChunkId,
        chunk_modules: IndexSet<ModuleId>,
    ) -> BoxSource {
        let module_sources = chunk_modules
            .iter()
            .map(|module_id| self.render_module(state, chunk_id, *module_id));
        let concat_source = ConcatSource::new(module_sources);
        concat_source.boxed()
    }
    pub fn render_module(
        &self,
        state: &mut CodeGenerationState,
        _chunk_id: ChunkId,
        module_id: ModuleId,
    ) -> BoxSource {
        state.code_generation_results.module_id_to_generation_result[&module_id]
            .sources
            .get(&crate::module::SourceType::JavaScript)
            .expect("get source")
            .clone()
    }
}