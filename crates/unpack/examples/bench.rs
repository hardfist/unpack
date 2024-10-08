use rspack_resolver::ResolveOptions;
use std::path::PathBuf;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let current_file = file!();
    let context = PathBuf::from(current_file)
        .join("../fixtures")
        .canonicalize()
        .unwrap();
    // for local bench
    let context = PathBuf::from("/Users/bytedance/project/performance-compare-ext/apps/10000");
    let compiler_options: CompilerOptions = CompilerOptions {
        context: context.try_into().expect("expect utf8 path"),
        entry: vec![EntryItem {
            name: "main".to_string(),
            import: "./src/index.jsx".to_string(),
        }],
        resolve: ResolveOptions {
            extensions: vec![".js", ".ts", ".mjs", ".jsx"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            ..Default::default()
        },
    };
    let mut compiler = Compiler::new(compiler_options);
    compiler.build();
}
