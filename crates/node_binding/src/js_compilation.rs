use napi::bindgen_prelude::WeakReference;
use napi_derive::napi;
use crate::js_compiler::JsCompiler;


struct Compilation {

}

#[napi]
pub struct JsCompilation {
    compilation: Compilation,
    compiler: WeakReference<JsCompiler>
}
#[napi]
impl JsCompilation {
    #[napi(getter)]
    pub fn compiler(&self) -> WeakReference<JsCompiler>{
        self.compiler.clone()
    }
}