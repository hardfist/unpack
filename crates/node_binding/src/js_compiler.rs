use std::sync::Arc;
use camino::Utf8PathBuf;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;
use napi::tokio::task::block_in_place;
use napi_derive::napi;
use unpack::{compiler::{Compiler, CompilerOptions}, plugin::BoxPlugin};

use crate::js_plugin::JsPluginAdapter;

#[napi]
struct JsCompiler {
    inner: Compiler
}

#[napi]
impl JsCompiler {
    #[napi(constructor)]
    pub fn new(context: String,entry: String, plugins: Vec<JsPluginAdapter>) -> Self {
        let options = CompilerOptions {
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
        };
        let plugins = plugins
                .into_iter()
                .map(|x| Arc::new(x) as BoxPlugin)
                .collect();
        let compiler = Compiler::new(Arc::new(options), plugins);
        Self { inner: compiler }
    }
    #[napi]
    pub async unsafe fn build(&mut self) -> napi::Result<()>{
        block_in_place(|| {
            self.inner.build();
        });
        println!("build finished");
        Ok(())
    }
}