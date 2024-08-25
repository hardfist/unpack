use std::path::Path;

use camino::Utf8PathBuf;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let context = std::env!("CARGO_MANIFEST_DIR");
    let compiler_options: CompilerOptions = CompilerOptions {
        context: Utf8PathBuf::from(context),
        entry: EntryItem {
            name: "main".to_string(),
            import: "./index.ts".to_string(),
        },
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}
