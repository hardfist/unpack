use rspack_resolver::ResolveOptions;
use std::{path::PathBuf, sync::Arc};
use tokio::runtime::Builder;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};
fn main() {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(4)
        .build()
        .unwrap();
    rt.block_on(async {
        let current_file = file!();
        dbg!(current_file);
        let context = PathBuf::from(current_file)
            .join("/Users/bytedance/project/build-tools-performance")
            .canonicalize()
            .unwrap();
        let compiler_options: CompilerOptions = CompilerOptions {
            context: context.try_into().expect("expect utf8 path"),
            entry: vec![EntryItem {
                name: "main".to_string(),
                import: "./src/medium/index.jsx".to_string(),
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
        compiler.build().await;
    });
}
