use std::{
    path::PathBuf,
    sync::Arc,
};

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use rspack_resolver::ResolveOptions;
use unpack_core::compiler::{Compiler, CompilerOptions, EntryItem};
async fn build() {
    let root = env!("CARGO_MANIFEST_DIR");
    let context = PathBuf::from(root)
        .parent()
        .unwrap()
        .join("../../build-tools-performance/cases/react-10k")
        .canonicalize()
        .unwrap();
    let compiler_options: CompilerOptions = CompilerOptions {
        context: context.clone().try_into().expect("expect utf8 path"),
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
        output_dir: context.join("dist").try_into().expect("expect utf8 path"),
    };
    let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
    compiler.build().await;
}
fn module_graph_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("module_graph", |b| {
        b.iter(|| rt.block_on(build()))
    });
}

criterion_group!(benches, module_graph_benchmark);
criterion_main!(benches);
