use async_trait::async_trait;
use napi::bindgen_prelude::External;
use napi::tokio::sync::mpsc::unbounded_channel;
use napi::{
    bindgen_prelude::{Buffer, Promise},
    threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
    Either,
};
use napi_derive::napi;
use std::{fmt::Debug, future::IntoFuture, sync::Arc};
use unpack::errors::miette::Result;
use unpack::plugin::{CompilationCell, LoadArgs, Plugin, PluginContext, ResolveArgs};

use crate::js_compilation::JsCompilation;


#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_resolve: Option<ThreadsafeFunction<String, Fatal>>,
    pub on_load: Option<ThreadsafeFunction<String, Fatal>>,
    pub this_compilation: Option<ThreadsafeFunction<JsCompilation, Fatal>>,
}
impl Debug for JsPluginAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPluginAdapter").finish()
    }
}
#[async_trait]
impl Plugin for JsPluginAdapter {
    fn name(&self) -> &'static str {
        "js_plugin_adapter"
    }
    async fn this_compilation(&self, _ctx: Arc<PluginContext>, compilation: Arc<CompilationCell>) {
        
        let compilation = JsCompilation::from_compilation(External::new(compilation));
        let (send, mut recv) = unbounded_channel();
        let Some(callback) = &self.this_compilation else {
            return ();
        };
        callback.call_with_return_value(
            compilation,
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret:()| {
                send.send(());
                Ok(())
            },
        );
        recv.recv().await.unwrap();
    }
    async fn load(&self, _ctx: Arc<PluginContext>, args: LoadArgs) -> Result<Option<Vec<u8>>> {
        let (send, mut recv) = unbounded_channel();
        let Some(callback) = &self.on_load else {
            return Ok(None);
        };
        callback.call_with_return_value(
            args.path.to_string(),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret: Either<Option<Buffer>, Promise<Option<Buffer>>>| {
                let _ = send.send(ret);
                Ok(())
            },
        );

        let result = recv.recv().await.unwrap();
        let result = match result {
            Either::A(s) => s,
            Either::B(s) => {
                (s.into_future()).await.unwrap()
                // use pollster::block_on;
                // block_on(s.into_future()).unwrap()
            }
        };
        Ok(result.map(|x| x.into()))
    }
    async fn resolve(&self, _ctx: Arc<PluginContext>, args: ResolveArgs) -> Result<Option<String>> {
        let (send, mut recv) = unbounded_channel();
        let Some(callback) = &self.on_resolve else {
            return Ok(None);
        };
        callback.call_with_return_value(
            args.path.to_string(),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret: Option<String>| {
                let _ = send.send(ret);
                Ok(())
            },
        );

        let result = recv.recv().await.unwrap();
        Ok(result)
    }
}
