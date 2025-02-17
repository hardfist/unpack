mod options;
use std::mem;
use std::sync::Arc;

pub use options::CompilerOptions;
pub use options::EntryItem;
use swc_core::common::plugin;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::BoxPlugin;
use crate::plugin::PluginContext;
use crate::plugin::PluginDriver;

pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<BoxPlugin>,
    compilation: Compilation
}

impl Compiler {
    pub fn new(options: Arc<CompilerOptions>, plugins: Vec<BoxPlugin>) -> Self {
        let plugin_driver = PluginDriver {
            plugins: plugins.clone(),
            plugin_context: Arc::new(PluginContext {
                options: options.clone()
            })
        };
        let compilation = Compilation::new(options.clone(), Arc::new(plugin_driver));
        Self { options, plugins ,compilation}
    }
    pub async fn build(&mut self) {
        let compilation = &mut self.compilation;
        let scanner_state = compilation.scan().await;
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
        compilation.diagnostics.extend(mem::take(&mut code_generation_state.diagnostics));
        let asset_state = compilation.create_chunk_asset(&mut code_generation_state);
        
        self.emit_assets(asset_state);
        
        if !self.compilation.diagnostics.is_empty() {
            for diag in &self.compilation.diagnostics {
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
