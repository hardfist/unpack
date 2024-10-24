mod options;
use std::mem;
use std::sync::Arc;

pub use options::CompilerOptions;
pub use options::EntryItem;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::BoxPlugin;
use crate::plugin::ResolveArgs;
use crate::plugin::Plugin;
use crate::plugin::PluginContext;
use crate::plugin::PluginDriver;

pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<BoxPlugin>
}

impl Compiler {
    pub fn new(options: Arc<CompilerOptions>, plugins: Vec<BoxPlugin>) -> Self {
        Self { options, plugins }
    }
    pub fn build(&mut self) {
        let plugin_driver = PluginDriver {
            plugins: self.plugins.clone(),
            plugin_context: Arc::new(PluginContext {
                options: self.options.clone()
            })
        };
        let mut compilation = Compilation::new(self.options.clone(), Arc::new(plugin_driver));
        let scanner_state = compilation.scan();
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
        compilation.diagnostics.extend(mem::take(&mut code_generation_state.diagnostics));
        let asset_state = compilation.create_chunk_asset(&mut code_generation_state);
        self.emit_assets(asset_state);
        
        if !compilation.diagnostics.is_empty() {
            for diag in compilation.diagnostics {
                println!("{:?}", diag);
            }
        }
    }
    pub fn emit_assets(&self, asset_state: ChunkAssetState) {
        for (_name, _source) in asset_state.assets {
            // std::fs::write(name, source.buffer().as_ref());
        }
    }
}
