use rspack_resolver::ResolveOptions;
use std::{path::PathBuf, sync::Arc};
use unpack::{
    compiler::{Compiler, CompilerOptions, EntryItem},
    tracing::init_tracing,
};

#[tokio::main]
async fn main() {
    let guard = init_tracing();
    let current_file = file!();
    let context = PathBuf::from(current_file)
        .join("../fixtures")
        .canonicalize()
        .unwrap();
    let compiler_options: CompilerOptions = CompilerOptions {
        output_dir: context.join("dist").try_into().expect("expect utf8 path"),
        context: context.try_into().expect("expect utf8 path"),
        entry: vec![
            EntryItem {
                name: "main".to_string(),
                import: "./src/index.mjs".to_string(),
            },
            EntryItem {
                name: "other".to_string(),
                import: "./src/other.mjs".to_string(),
            },
        ],
        resolve: ResolveOptions {
            extensions: vec![".js", ".ts", ".mjs"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            ..Default::default()
        },
    };
    tracing::debug!("compiler_options: {:?}", compiler_options);
    let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
    compiler.build().await;
    drop(guard);
}
