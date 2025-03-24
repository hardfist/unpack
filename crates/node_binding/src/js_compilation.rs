use std::sync::Arc;

use napi::bindgen_prelude::ObjectFinalize;
use napi_derive::napi;
use unpack::plugin::CompilationCell;

#[napi(custom_finalize)]
pub struct JsCompilation {
    compilation: Arc<CompilationCell>,
    id: u32,
}

impl JsCompilation {
    pub fn from_compilation(compilation: Arc<CompilationCell>) -> Self {
        let id = unsafe { &*compilation.get() }.id;
        Self {
            compilation: compilation,
            id: id.0,
        }
    }
}
impl ObjectFinalize for JsCompilation {
    fn finalize(self, mut env: napi::Env) -> napi::Result<()> {
        println!("JsCompilation:{} finalize", self.id);

        Ok(())
    }
}
