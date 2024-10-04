mod module_id;
mod normal_module;
mod ast;
use std::fmt::Debug;
use std::sync::Arc;

use ast::AST;
pub use normal_module::*;
pub use module_id::*;

use crate::compiler::CompilerOptions;

use crate::dependency::BoxDependency;
use crate::errors::miette::Result;

#[derive(Debug)]
pub(crate) struct BuildResult {
    dependencies: Vec<BoxDependency>
}
pub(crate) struct BuildContext {
   pub(crate) options: Arc<CompilerOptions>
}
pub(crate) trait Module: Debug {
    fn build(&mut self,build_context: BuildContext) -> Result<BuildResult>;
}

pub(crate) type BoxModule = Box<dyn Module>;
#[derive(Debug)]
pub struct ModuleIdentifier(String);
// #[derive(Debug)]
// pub struct NormalModuleDraft {
//     diagnostics: Diagnostics,
//     original_source: Option<BoxSource>,
// }
