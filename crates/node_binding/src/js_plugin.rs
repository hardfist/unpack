use std::{fmt::Debug, sync::Arc};
use napi::{
    threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
    Either,
};
use unpack::errors::miette::Result;
use std::sync::mpsc::channel;
use unpack::plugin::{LoadArgs, Plugin, PluginContext, ResolveArgs};

#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_resolve: Option<ThreadsafeFunction<String,Fatal>>,
    pub on_load: Option<ThreadsafeFunction<String,Fatal>>
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
    fn load(&self,_ctx: Arc<PluginContext>, args: LoadArgs) -> Result<Option<Vec<u8>>> {
        let (send, recv) = channel();
        let Some(callback) = &self.on_load else {
            return Ok(None);
        };
        callback.call_with_return_value(
            args.path.to_string(),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret: Option<Vec<u8>>| {
                let _ = send.send(ret);
                Ok(())
            },
        );

        let result = recv.recv().unwrap();
        Ok(result)
    }
    fn resolve(
        &self,
        _ctx: Arc<PluginContext>,
        args: ResolveArgs,
    ) -> Result<Option<String>> {
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
