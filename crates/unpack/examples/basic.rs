use rspack_resolver::ResolveOptions;
use std::path::PathBuf;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let current_file = file!();
    let context = PathBuf::from(current_file)
        .join("../fixtures")
        .canonicalize()
        .unwrap();
    let compiler_options: CompilerOptions = CompilerOptions {
        context: context.try_into().expect("expect utf8 path"),
        entry: EntryItem {
            name: "main".to_string(),
            import: "./src/index.mjs".to_string(),
        },
        resolve: ResolveOptions {
            extensions: vec![".js",".ts",".mjs"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            ..Default::default()
        },
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}
