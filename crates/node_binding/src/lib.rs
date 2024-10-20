#![deny(clippy::all)]
mod js_plugin;
use camino::Utf8PathBuf;
use js_plugin::JsPluginAdapter;
use unpack::compiler::EntryItem;
use unpack::plugin::Plugin;
use unpack::resolver::ResolveOptions;
use unpack::{bundler::unpack, compiler::CompilerOptions};
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn build(context: String, entry: String, plugins: Vec<JsPluginAdapter>) -> napi::Result<()> {
    std::thread::spawn(move || {
        unpack(CompilerOptions {
            context: Utf8PathBuf::from(context),
            entry: vec![EntryItem {
                name: "main".to_string(),
                import: entry,
            }],
            resolve: ResolveOptions {
                extensions: vec![".js", ".ts", ".mjs"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
                ..Default::default()
            },
        }, plugins.into_iter().map(|x| Box::new(x) as Box<dyn Plugin>).collect());
    });

    Ok(())
}
