mod options;
use std::sync::Arc;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::BoxPlugin;
use crate::plugin::PluginContext;
use crate::plugin::PluginDriver;
use crate::scheduler::CompilerContext;
use crate::scheduler::COMPILER_CONTEXT;
use camino::Utf8Path;
pub use options::CompilerOptions;
pub use options::EntryItem;
use rspack_sources::BoxSource;

pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<BoxPlugin>,
    last_compilation: Option<Box<Compilation>>,
    plugin_driver: Arc<PluginDriver>,
    compiler_context: Arc<CompilerContext>,
}

impl Compiler {
    pub fn new(options: Arc<CompilerOptions>, plugins: Vec<BoxPlugin>) -> Self {
        let plugin_driver = Arc::new(PluginDriver {
            plugins: plugins.clone(),
            plugin_context: Arc::new(PluginContext {
                options: options.clone(),
            }),
        });

        Self {
            options,
            plugins,
            last_compilation: None,
            plugin_driver: plugin_driver.clone(),
            compiler_context: Arc::new(CompilerContext::new()),
        }
    }
    pub async fn build(&mut self) {
        COMPILER_CONTEXT
            .scope(self.compiler_context.clone(), async {
                println!(
                    "Compiler build started with ID: {}",
                    self.compiler_context.get_compiler_id()
                );
                let mut compilation = Box::new(Compilation::new(
                    self.options.clone(),
                    self.plugin_driver.clone(),
                ));

                self.plugin_driver
                    .run_compilation_hook(&mut *compilation)
                    .await;

                let memory_manager = self.compiler_context.get_memory_manager();
                let scanner_result = compilation.scan(memory_manager).await;
                let linker_result = compilation.link(
                    scanner_result.entries,
                    scanner_result.module_graph,
                    memory_manager,
                );
                let mut code_generation_state = compilation.code_generation(
                    linker_result,
                    memory_manager,
                    &scanner_result.collect_modules,
                );

                // compilation
                //     .diagnostics
                //     .write().unwrap()
                //     .extend(mem::take(&mut code_generation_state.diagnostics));
                let asset_state = compilation.create_chunk_asset(&mut code_generation_state);

                self.emit_assets(asset_state).await;
                if !compilation.diagnostics.read().unwrap().is_empty() {
                    for diag in compilation.diagnostics.read().unwrap().iter() {
                        println!("{diag:?}");
                    }
                }
                self.last_compilation = Some(compilation);
                println!("Compilation finished");
            })
            .await;
    }
    pub async fn emit_assets(&self, asset_state: ChunkAssetState) {
        for (filename, asset) in asset_state.assets {
            self.emit_asset(&self.options.output_dir, &filename, asset)
                .await;
        }
    }
    async fn emit_asset(&self, output_dir: &Utf8Path, filename: &str, asset: BoxSource) {
        let full_path = output_dir.join(filename);
        if !full_path.parent().unwrap().exists() {
            std::fs::create_dir_all(full_path.parent().unwrap()).unwrap();
        }
        std::fs::write(full_path, asset.buffer().as_ref()).unwrap();
    }
}
