use crate::js_plugin::JsPluginAdapter;
use camino::Utf8PathBuf;
use napi::bindgen_prelude::Reference;
use napi::Env;
use napi_derive::napi;
use std::sync::Arc;
use unpack::compilation::Compilation;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;
use unpack::{
    compiler::{Compiler, CompilerOptions},
    plugin::BoxPlugin,
};

#[napi]
pub struct JsCompiler {
    inner: Option<Compiler>,
}

#[napi]
impl JsCompiler {
    #[napi(constructor)]
    pub fn new(
        env: Env,
        context: String,
        entry: String,
        mut plugins: Vec<JsPluginAdapter>,
    ) -> Self {
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
        // unref napi handles to avoid hang problem
        for plugin in plugins.iter_mut() {
            if let Some(resolve) = &mut plugin.on_resolve {
                resolve.unref(&env).unwrap();
            }
            if let Some(load) = &mut plugin.on_load {
                load.unref(&env).unwrap();
            }
            if let Some(this_compilation) = &mut plugin.this_compilation {
                this_compilation.unref(&env).unwrap();
            }
        }

        let plugins = plugins
            .into_iter()
            .map(|x| Arc::new(x) as BoxPlugin)
            .collect();
        let compiler = Compiler::new(Arc::new(options), plugins);
        Self {
            inner: Some(compiler),
        }
    }
    #[napi]
    pub async unsafe fn build(&mut self) -> napi::Result<()> {
        let mut compiler = self.inner.take().unwrap();
        let compiler = napi::tokio::spawn(async {
            compiler.build().await;
            compiler
        })
        .await
        .unwrap();
        self.inner = Some(compiler);

        Ok(())
    }
}
