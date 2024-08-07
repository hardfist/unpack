
use std::path::{Path};

pub use unpack::unpack;
use unpack::Compiler;
use unpack::CompilerOptions;
use unpack::EntryItem;
fn main(){
    let context = std::env!("CARGO_MANIFEST_DIR");
    let compiler_options: CompilerOptions = CompilerOptions {
        context: Path::new(context).to_path_buf(),
        entry:  EntryItem {
            name: "main".to_string(),
            import: "./index.ts".to_string()
        }
        
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}