use rspack_resolver::ResolveOptions;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::runtime::Builder;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};
fn main() {
    if std::env::var("UNPACK_PROFILE").is_ok() {
        let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();

        tracing_subscriber::registry().with(chrome_layer).init();
    }

    let rt = Builder::new_multi_thread()
        .enable_all()
        .disable_lifo_slot()
        .max_blocking_threads(8)
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-{}", id)
        })
        .build()
        .unwrap();
    rt.block_on(async {
        let current_file = file!();
        dbg!(current_file);
        let context = PathBuf::from(current_file)
            .parent()
            .unwrap()
            .join("../../../benchmark/build-tools-performance")
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
