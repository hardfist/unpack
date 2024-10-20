use napi::{ bindgen_prelude::{block_on, Promise}, threadsafe_function::ThreadsafeFunction, tokio::sync::oneshot, Either};
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
                move |ret: Either<String,Promise<String>>| {
                    let _ = send.send(ret);
                    Ok(())
                },
            );
        
        let result = recv.blocking_recv().unwrap();
        let s = match result {
            Either::A(s)=> {
                s
            },
            Either::B(s) => {
                block_on(s).unwrap()
            }
        };
        Ok(Some(s))
    }
}
