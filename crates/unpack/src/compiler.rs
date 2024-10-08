mod options;
use std::sync::Arc;

pub use options::CompilerOptions;
pub use options::EntryItem;

use crate::compilation::ChunkAssetState;
use crate::compilation::Compilation;

pub struct Compiler {
    #[allow(dead_code)]
    options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }
    pub fn build(&mut self) {
        let mut compilation = Compilation::new(Arc::new(self.options.clone()));
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
        for (name, source) in asset_state.assets {
            // std::fs::write(name, source.buffer().as_ref());
        }
    }
}
