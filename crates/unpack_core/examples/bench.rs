use rspack_resolver::ResolveOptions;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::runtime::Builder;
use tracing::level_filters::LevelFilter;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};
use unpack_core::compiler::{Compiler, CompilerOptions, EntryItem};
fn main() {
    let _guard = match std::env::var("UNPACK_PROFILE") {
        Ok(filter) => {
            let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
            let env_filter = EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .parse(filter)
                .expect("invalid filter");
            tracing_subscriber::registry()
                .with(chrome_layer.with_filter(env_filter))
                .init();
            Some(guard)
        }
        Err(_) => None,
    };

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
            output_dir: context.join(dist).try_into().expect("expect utf8 path"),
        };
        let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
        compiler.build().await;
    });
}
