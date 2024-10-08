#![deny(clippy::all)]
use std::ptr::copy_nonoverlapping;

use camino::Utf8PathBuf;
use napi::threadsafe_function::ThreadsafeFunction;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;

use unpack::{bundler::unpack, compiler::CompilerOptions};
use napi::bindgen_prelude::*;
#[macro_use]
extern crate napi_derive;

#[napi]
pub async fn build(context: String,entry:String, callback: ThreadsafeFunction<u32,String>) ->napi::Result<()> {
  let res = callback.call_async(Ok(123)).await.unwrap();
  println!("res:{:?}",res);
  let result = unpack(CompilerOptions{
    context: Utf8PathBuf::from(context),
    entry: vec![EntryItem{
      name: "main".to_string(),
      import: entry
    }],
    resolve: ResolveOptions {
            extensions: vec![".js", ".ts", ".mjs"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            ..Default::default()
        },
  });
  Ok(())
}
