mod options;
use std::sync::Arc;

pub use options::CompilerOptions;
pub use options::EntryItem;

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
        println!("start build");
        let mut compilation = Compilation::new(Arc::new(self.options.clone()));
        let scanner_state = compilation.scan();
        let linker_state = compilation.link(scanner_state);
        let mut code_generation_state = compilation.code_generation(linker_state);
        let asset_state = compilation.create_chunk_asset(&mut code_generation_state);
        dbg!(asset_state);
        println!("finish build");
        if !compilation.diagnostics.is_empty() {
            for diag in compilation.diagnostics {
                println!("{:?}", diag);
            }
        }
    }
}
