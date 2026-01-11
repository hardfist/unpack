use std::ptr::NonNull;

use napi::bindgen_prelude::ObjectFinalize;
use napi_derive::napi;
use unpack_core::compilation::Compilation;

#[napi(custom_finalize)]
pub struct JsCompilation {
    #[allow(dead_code)]
    compilation: NonNull<Compilation>,
    id: u32,
}

impl JsCompilation {
    pub fn from_compilation(compilation: NonNull<Compilation>) -> Self {
        let id = unsafe { compilation.as_ref().id };
        Self {
            compilation,
            id: id.0,
        }
    }
}
impl ObjectFinalize for JsCompilation {
    fn finalize(self, _env: napi::Env) -> napi::Result<()> {
        println!("JsCompilation:{} finalize", self.id);

        Ok(())
    }
}
