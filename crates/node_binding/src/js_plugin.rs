use napi::{ threadsafe_function::ThreadsafeFunction, tokio::sync::oneshot};
use unpack::plugin::{LoadArgs, Plugin};

#[napi(object, object_to_js = false)]
pub struct JsPluginAdapter {
    pub on_load: ThreadsafeFunction<String>,
}
impl Plugin for JsPluginAdapter {
    fn name(&self) -> &'static str {
        "js_plugin_adapter"
    }
    fn load(
        &self,
        _ctx: unpack::plugin::PluginContext,
        args: LoadArgs,
    ) -> unpack::errors::miette::Result<Option<String>> {
        let (send, recv) = oneshot::channel();
        let callback = self.on_load.clone();
            callback.call_with_return_value(
                Ok(args.path.to_string()),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
                move |ret: String| {
                    send.send(ret).unwrap();
                    Ok(())
                },
            );
        let result = recv.blocking_recv().unwrap();
        Ok(Some(result))
    }
}
