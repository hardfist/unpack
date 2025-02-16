use std::sync::Arc;
use camino::Utf8PathBuf;
use napi::tokio;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;
use napi_derive::napi;
use unpack::{compiler::{Compiler, CompilerOptions}, plugin::BoxPlugin};

use crate::js_plugin::JsPluginAdapter;

#[napi]
struct JsCompiler {
    inner: Option<Compiler>
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
        Self { inner: Some(compiler)}
    }
    #[napi]
    pub async unsafe fn build(&mut self) -> napi::Result<()>{
        let mut compiler = self.inner.take().unwrap();
        // let compiler = napi::tokio::spawn(async {
        //     compiler.build().await;
        //     compiler
        // }).await.unwrap();
       self.inner = Some(compiler);
        Ok(())
    }
   
}