use super::CodeGenerationContext;
use super::NormalModule;
use crate::compiler::CompilerOptions;
use crate::memory_manager::arena::Idx;
use crate::runtime::RuntimeGlobals;
use async_trait::async_trait;
use camino::Utf8Path;
use index_vec::define_index_type;
use index_vec::IndexVec;
use rspack_sources::BoxSource;
use swc_core::ecma::utils::Type::Bool;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::dependency::BoxDependency;
use crate::dependency::DependenciesBlock;
use crate::errors::miette::Result;
use crate::plugin::PluginDriver;

#[derive(Debug)]
pub struct BuildResult {
    pub module_dependencies: Vec<BoxDependency>,
}
pub struct BuildContext {
    pub options: Arc<CompilerOptions>,
    pub plugin_driver: Arc<PluginDriver>,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    JavaScript,
    Css,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone)]
pub struct CodeGenerationResult {
    pub sources: HashMap<SourceType, BoxSource>,
    pub runtime_requirements: RuntimeGlobals,
}
impl CodeGenerationResult {
    pub fn add(&mut self, source_type: SourceType, source: BoxSource) {
        self.sources.insert(source_type, source);
    }
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
    fn source_types(&self) -> &[SourceType];
}

pub type BoxModule = Box<dyn Module>;
pub type ModuleId = Idx<BoxModule>;