use std::path::Path;

use camino::Utf8PathBuf;
use rspack_resolver::ResolveOptions;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let context = std::env!("CARGO_MANIFEST_DIR");
    let compiler_options: CompilerOptions = CompilerOptions {
        context: Utf8PathBuf::from(context),
        entry: EntryItem {
            name: "main".to_string(),
            import: "./index.ts".to_string(),
          
        },
        resolve: ResolveOptions::default()
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}
