mod ast;
mod connection;
mod module_graph;
mod module_id;
mod module_scanner;
mod normal_module;
use camino::Utf8Path;
pub use connection::*;
pub use module_graph::*;
pub use module_id::*;
pub use module_scanner::*;
pub use normal_module::*;
use std::fmt::Debug;
use std::sync::Arc;

use crate::compiler::CompilerOptions;

use crate::dependency::BoxDependency;
use crate::errors::miette::Result;

#[derive(Debug)]
pub struct BuildResult {
    pub dependencies: Vec<BoxDependency>,
}
pub struct BuildContext {
    pub options: Arc<CompilerOptions>,
}
pub trait Module: Debug {
    fn build(&mut self, build_context: BuildContext) -> Result<BuildResult>;
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
}

pub type BoxModule = Box<dyn Module>;
#[derive(Debug)]
pub struct ModuleIdentifier(String);
// #[derive(Debug)]
// pub struct NormalModuleDraft {
//     diagnostics: Diagnostics,
//     original_source: Option<BoxSource>,
// }
