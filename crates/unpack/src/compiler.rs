mod options;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use camino::Utf8PathBuf;
pub use options::CompilerOptions;
pub use options::EntryItem;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;
use crate::plugin::LoadArgs;
use crate::plugin::Plugin;
use crate::plugin::PluginContext;

pub struct Compiler {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    plugins: Vec<Box<dyn Plugin>>
}

impl Compiler {
    pub fn new(options: Arc<CompilerOptions>,plugins: Vec<Box<dyn Plugin>>) -> Self {
        Self { options, plugins }
    }
    pub fn build(&mut self) {
        let plugin_context = PluginContext{
            options: self.options.clone()
        };
        for plugin in &self.plugins {
            let result = plugin.load(plugin_context.clone(), LoadArgs {
                path: self.options.context.clone()
            }).unwrap();
            dbg!(result);
        }
        let mut compilation = Compilation::new(self.options.clone());
        let scanner_state = compilation.scan();
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
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
