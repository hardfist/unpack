use std::{cell::UnsafeCell, ptr::NonNull, sync::Arc};

use napi::bindgen_prelude::External;
use napi_derive::napi;
use unpack::{compilation::Compilation, plugin::CompilationCell};

#[napi]
pub struct JsCompilation {
    compilation: External<Arc<CompilationCell>>
}

impl JsCompilation {
    pub fn from_compilation(compilation: External<Arc<CompilationCell>>) -> Self{
        Self {
            compilation: compilation
        }
    }
}