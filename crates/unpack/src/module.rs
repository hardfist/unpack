mod ast;
mod module_id;
mod normal_module;
use camino::Utf8Path;
pub use module_id::*;
pub use normal_module::*;
use std::fmt::Debug;
use std::sync::Arc;

use crate::compiler::CompilerOptions;

use crate::dependency::BoxDependency;
use crate::errors::miette::Result;

#[derive(Debug)]
pub(crate) struct BuildResult {
    pub(crate) dependencies: Vec<BoxDependency>,
}
pub(crate) struct BuildContext {
    pub(crate) options: Arc<CompilerOptions>,
}
pub(crate) trait Module: Debug {
    fn build(&mut self, build_context: BuildContext) -> Result<BuildResult>;
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
}

pub(crate) type BoxModule = Box<dyn Module>;
#[derive(Debug)]
pub struct ModuleIdentifier(String);
// #[derive(Debug)]
// pub struct NormalModuleDraft {
//     diagnostics: Diagnostics,
//     original_source: Option<BoxSource>,
// }
