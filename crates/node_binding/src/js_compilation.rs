use std::{ptr::NonNull, sync::Arc};

use napi::{bindgen_prelude::WeakReference, Ref};
use napi_derive::napi;
use crate::js_compiler::JsCompiler;


struct Compilation {

}

#[napi]
pub struct JsCompilation {
    //pub compilation: Arc<Compilation>
    // compiler: WeakReference<JsCompiler>
}
#[napi]
impl JsCompilation {
    // #[napi(getter)]
    // pub fn compiler(&self) -> WeakReference<JsCompiler>{
    //     self.compiler.clone()
    // }
    
}