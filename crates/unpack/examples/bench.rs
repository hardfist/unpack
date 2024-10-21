use rspack_resolver::ResolveOptions;
use std::{path::PathBuf, sync::Arc};
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};

fn main() {
    let current_file = file!();
    dbg!(current_file);
    let context = PathBuf::from(current_file)
        .join("../../../../benchmark/performance-compare-ng/apps/10000")
        .canonicalize()
        .unwrap();
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
    let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
    compiler.build();
}
