use rspack_resolver::ResolveOptions;
use tracing_chrome::{ ChromeLayerBuilder};
use tracing_subscriber::layer::SubscriberExt;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::{runtime::Builder, time::sleep};
use unpack::compiler::{Compiler, CompilerOptions, EntryItem};
use tracing_subscriber::util::SubscriberInitExt;
fn main() {
    
    let (chrome_layer,_guard) = ChromeLayerBuilder::new().build();

    tracing_subscriber::registry().with(chrome_layer).init();

    let rt = Builder::new_multi_thread()
        .enable_all()
        .disable_lifo_slot()
        .max_blocking_threads(4)
        .build()
        .unwrap();
    rt.block_on(async {
        let current_file = file!();
        dbg!(current_file);
        let context = PathBuf::from(current_file).parent().unwrap()
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
        sleep(Duration::from_secs(1000)).await;
    });
}
