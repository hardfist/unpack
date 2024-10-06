use super::NormalModule;
use camino::Utf8Path;
use index_vec::define_index_type;
use index_vec::IndexVec;
use std::fmt::Debug;
use std::sync::Arc;

use crate::compiler::CompilerOptions;

use crate::dependency::BoxDependency;
use crate::dependency::DependenciesBlock;
use crate::errors::miette::Result;



#[derive(Debug)]
pub struct BuildResult {
    pub dependencies: Vec<BoxDependency>,
}
pub struct BuildContext {
    pub options: Arc<CompilerOptions>,
}
pub trait Module: Debug + DependenciesBlock {
    fn build(&mut self, build_context: BuildContext) -> Result<BuildResult>;
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
}

pub type BoxModule = Box<dyn Module>;

define_index_type! {
    pub struct ModuleId = u32;
}

pub type ModuleVec = IndexVec<ModuleId, NormalModule>;
