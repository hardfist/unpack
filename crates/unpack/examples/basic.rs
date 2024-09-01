use std::path::{Path, PathBuf};
use rspack_resolver::ResolveOptions;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let current_file = file!();
    let context = PathBuf::from(current_file).join("../fixtures").canonicalize().unwrap();
    let compiler_options: CompilerOptions = CompilerOptions {
        context: context.try_into().expect("expect utf8 path"),
        entry: EntryItem {
            name: "main".to_string(),
            import: "./index.mjs".to_string(),
          
        },
        resolve: ResolveOptions::default()
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}
