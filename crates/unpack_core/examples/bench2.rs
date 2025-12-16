use rspack_resolver::ResolveOptions;
use unpack_core::compiler::{Compiler, CompilerOptions, EntryItem};
use std::{path::PathBuf, sync::Arc};
async fn build() {
    let root = env!("CARGO_MANIFEST_DIR");
    dbg!(root);
    let context = PathBuf::from(root)
        .join("../../benchmark/build-tools-performance")
        .canonicalize()
        .unwrap();
    dbg!(&context);
    let compiler_options: CompilerOptions = CompilerOptions {
        context: context.clone().try_into().expect("expect utf8 path"),
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
        output_dir: context.join("dist").try_into().expect("expect utf8 path"),
    };
    let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
    compiler.build().await;
}

#[tokio::main]
async fn main(){
    build().await;
}