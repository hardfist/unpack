#![deny(clippy::all)]
mod js_plugin;
mod js_compilation;
mod js_compiler;
use camino::Utf8PathBuf;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;