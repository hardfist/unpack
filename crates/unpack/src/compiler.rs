mod options;
use std::mem;
use std::sync::Arc;

pub use options::CompilerOptions;
pub use options::EntryItem;
use swc_core::common::plugin;
use std::cell::UnsafeCell;
use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::BoxPlugin;
use crate::plugin::CompilationCell;
use crate::plugin::PluginContext;
use crate::plugin::PluginDriver;

pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<BoxPlugin>,
    compilation: Arc<CompilationCell>,
    plugin_driver: Arc<PluginDriver>
}

impl Compiler {
    pub fn new(options: Arc<CompilerOptions>, plugins: Vec<BoxPlugin>) -> Self {
        let plugin_driver = Arc::new(PluginDriver {
            plugins: plugins.clone(),
            plugin_context: Arc::new(PluginContext {
                options: options.clone()
            })
        });
        
        let compilation = Arc::new(CompilationCell::new(Compilation::new(options.clone(), plugin_driver.clone())));
        Self { options, plugins , compilation: compilation,plugin_driver: plugin_driver.clone()}
    }
    pub async fn build(&mut self) {
        self.plugin_driver.run_compilation_hook(self.compilation.clone()).await;
        let compilation = unsafe { &mut *self.compilation.get() };
        let scanner_state = compilation.scan().await;
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
        compilation.diagnostics.extend(mem::take(&mut code_generation_state.diagnostics));
        let asset_state = compilation.create_chunk_asset(&mut code_generation_state);
        
        self.emit_assets(asset_state);
        let compilation: &Compilation = unsafe{&*self.compilation.get()};
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
