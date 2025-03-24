use super::CodeGenerationContext;
use super::NormalModule;
use crate::compiler::CompilerOptions;
use async_trait::async_trait;
use camino::Utf8Path;
use index_vec::define_index_type;
use index_vec::IndexVec;
use rspack_sources::BoxSource;
use std::fmt::Debug;
use std::sync::Arc;

use crate::dependency::BoxDependency;
use crate::dependency::BoxDependencyTemplate;
use crate::dependency::DependenciesBlock;
use crate::errors::miette::Result;
use crate::plugin::PluginDriver;

#[derive(Debug)]
pub struct BuildResult {
    pub module_dependencies: Vec<BoxDependency>,
    pub presentational_dependencies: Vec<BoxDependencyTemplate>,
}
pub struct BuildContext {
    pub options: Arc<CompilerOptions>,
    pub plugin_driver: Arc<PluginDriver>,
}
#[derive(Debug)]
pub struct CodeGenerationResult {
    pub source: BoxSource,
}
#[async_trait]
pub trait Module: Debug + DependenciesBlock + Send + Sync {
    fn identifier(&self) -> &str;
    async fn build(&mut self, build_context: BuildContext) -> Result<BuildResult>;
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
    fn code_generation(
        &self,
        code_generation_context: CodeGenerationContext,
    ) -> Result<CodeGenerationResult>;
}

pub type BoxModule = Box<dyn Module>;

define_index_type! {
    pub struct ModuleId = u32;
}

pub type ModuleVec = IndexVec<ModuleId, NormalModule>;
