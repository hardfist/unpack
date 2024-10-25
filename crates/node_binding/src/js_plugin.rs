use async_std::task::block_on;
use napi::{
    bindgen_prelude::{Buffer, Promise},
    threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
    Either,
};
use async_trait::async_trait;
use std::sync::mpsc::channel;
use std::{fmt::Debug, future::IntoFuture, sync::Arc};
use unpack::errors::miette::Result;
use unpack::plugin::{LoadArgs, Plugin, PluginContext, ResolveArgs};

#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_resolve: Option<ThreadsafeFunction<String, Fatal>>,
    pub on_load: Option<ThreadsafeFunction<String, Fatal>>,
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
    async fn load(&self, _ctx: Arc<PluginContext>, args: LoadArgs) -> Result<Option<Vec<u8>>> {
        let (send, recv) = channel();
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

        let result = recv.recv().unwrap();
        let result = match result {
            Either::A(s) => s,
            Either::B(s) => {
                let res = block_on(s.into_future()).unwrap();
                res
            }
        };
        Ok(result.map(|x| x.into()))
    }
    fn resolve(&self, _ctx: Arc<PluginContext>, args: ResolveArgs) -> Result<Option<String>> {
        let (send, recv) = channel();
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

        let result = recv.recv().unwrap();
        Ok(result)
    }
}
