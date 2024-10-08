#![deny(clippy::all)]
use camino::Utf8PathBuf;
use unpack::compiler::EntryItem;

use unpack::resolver::ResolveOptions;
use unpack::{bundler::unpack, compiler::CompilerOptions};
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn build(context: String,entry:String) ->napi::Result<()> {
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
