#![deny(clippy::all)]
mod js_plugin;
mod js_compilation;
mod js_compiler;
use camino::Utf8PathBuf;
use js_plugin::JsPluginAdapter;
use napi::bindgen_prelude::spawn_blocking;
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
    spawn_blocking(move || {
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
    }).await.unwrap();
    Ok(())
}
