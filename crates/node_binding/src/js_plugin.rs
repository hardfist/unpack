use std::{fmt::Debug, future::IntoFuture, sync::Arc};
use async_std::task::block_on;
use napi::{
    bindgen_prelude::{ Promise},
    threadsafe_function::ThreadsafeFunction,
    tokio::sync::oneshot,
    Either,
};
use unpack::plugin::{LoadArgs, Plugin};

#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_load: ThreadsafeFunction<String>,
}
impl Debug for JsPluginAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPluginAdapter").finish()
    }
}
impl Plugin for JsPluginAdapter {
    fn name(&self) -> &'static str {
        "js_plugin_adapter"
    }
    fn load(
        &self,
        _ctx: Arc<unpack::plugin::PluginContext>,
        args: LoadArgs,
    ) -> unpack::errors::miette::Result<Option<String>> {
        let (send, recv) = oneshot::channel();
        let callback = self.on_load.clone();
        callback.call_with_return_value(
            Ok(args.path.to_string()),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret: Either<Option<String>, Promise<Option<String>>>| {
                let _ = send.send(ret);
                Ok(())
            },
        );

        let result = block_on(recv.into_future()).unwrap();
        let s = match result {
            Either::A(s) => s,
            Either::B(s) => block_on(s).unwrap(),
        };
        Ok(s)
    }
}
