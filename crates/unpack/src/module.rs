mod module_id;
mod normal_module;
use std::fmt::Debug;
use std::sync::Arc;

pub use normal_module::*;
pub use module_id::*;

use crate::compiler::CompilerOptions;

use crate::errors::miette::Result;

#[derive(Debug)]
pub(crate) struct BuildResult {

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
