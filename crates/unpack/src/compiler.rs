mod options;
use std::mem;
use std::sync::Arc;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::BoxPlugin;
use crate::plugin::CompilationCell;
use crate::plugin::PluginContext;
use crate::plugin::PluginDriver;
pub use options::CompilerOptions;
pub use options::EntryItem;

impl Drop for Compiler {
    fn drop(&mut self) {
        println!("native Compiler dropped");
    }
}
pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<BoxPlugin>,
    last_compilation: Option<Arc<CompilationCell>>,
    plugin_driver: Arc<PluginDriver>,
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
        }
    }
    pub async fn build(&mut self) {
        let compilation = Arc::new(CompilationCell::new(Compilation::new(
            self.options.clone(),
            self.plugin_driver.clone(),
        )));
        self.last_compilation = Some(compilation.clone());
        self.plugin_driver
            .run_compilation_hook(compilation.clone())
            .await;
        let compilation = unsafe { &mut *compilation.get() };
        let scanner_state = compilation.scan().await;
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
        compilation
            .diagnostics
            .extend(mem::take(&mut code_generation_state.diagnostics));
        let asset_state = compilation.create_chunk_asset(&mut code_generation_state);

        self.emit_assets(asset_state);
        let compilation: &Compilation = unsafe { &*self.last_compilation.as_ref().unwrap().get() };
        if !compilation.diagnostics.is_empty() {
            for diag in &compilation.diagnostics {
                println!("{:?}", diag);
            }
        }
        println!("Compilation finished");
    }
    pub fn emit_assets(&self, asset_state: ChunkAssetState) {
        for (_name, _source) in asset_state.assets {
            // std::fs::write(name, source.buffer().as_ref());
        }
    }
}
