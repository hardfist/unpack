use std::{fmt::Debug, future::IntoFuture, sync::Arc};
use async_std::task::block_on;
use napi::{
    threadsafe_function::ThreadsafeFunction,
    Either,
};
use std::sync::mpsc::channel;
use unpack::plugin::{LoadArgs, Plugin};

#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_resolve: Option<ThreadsafeFunction<String>>
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
    fn resolve(
        &self,
        _ctx: Arc<unpack::plugin::PluginContext>,
        args: LoadArgs,
    ) -> unpack::errors::miette::Result<Option<String>> {
        let (send, recv) = channel();
        let Some(callback) = &self.on_resolve else {
            return Ok(None);
        };
        callback.call_with_return_value(
            Ok(args.path.to_string()),
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
