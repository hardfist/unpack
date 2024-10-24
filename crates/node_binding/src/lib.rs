#![deny(clippy::all)]
mod js_plugin;
use async_std::channel::bounded;
use async_std::task::block_on;
use camino::Utf8PathBuf;
use js_plugin::JsPluginAdapter;
use std::sync::Arc;
use unpack::compiler::EntryItem;
use unpack::plugin::BoxPlugin;
use unpack::resolver::ResolveOptions;
use unpack::{bundler::unpack, compiler::CompilerOptions};
#[macro_use]
extern crate napi_derive;

#[napi]
pub async fn build(
    context: String,
    entry: String,
    plugins: Vec<JsPluginAdapter>,
) -> napi::Result<()> {
    let (tx, rx) = bounded(1);
    std::thread::spawn(move || {
        unpack(
            CompilerOptions {
                context: Utf8PathBuf::from(context),
                entry: vec![EntryItem {
                    name: "main".to_string(),
                    import: entry,
                }],
                resolve: ResolveOptions {
                    extensions: vec![".js", ".ts", ".mjs", ".jsx"]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                    ..Default::default()
                },
            },
            plugins
                .into_iter()
                .map(|x| Arc::new(x) as BoxPlugin)
                .collect(),
        );
        tx.send_blocking(()).unwrap();
    });
    rx.recv().await.unwrap();
    Ok(())
}
