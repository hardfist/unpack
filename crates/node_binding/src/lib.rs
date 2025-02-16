#![deny(clippy::all)]
mod js_plugin;
mod js_compilation;
mod js_compiler;
use camino::Utf8PathBuf;
use js_plugin::JsPluginAdapter;
use napi::bindgen_prelude::spawn_blocking;
use std::sync::Arc;
use unpack::compiler::EntryItem;
use unpack::plugin::BoxPlugin;
use unpack::resolver::ResolveOptions;
use unpack::{bundler::unpack, compiler::CompilerOptions};
#[macro_use]
extern crate napi_derive;
