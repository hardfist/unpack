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
        compilation.scan();
        compilation.link();
        println!("finish build");
        if !compilation.diagnostics.is_empty() {
            for diag in compilation.diagnostics {
                println!("{:?}", diag);
            }
        }
    }
}
