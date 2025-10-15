use super::CodeGenerationContext;
use crate::compiler::CompilerOptions;
use crate::dependency::DependencyId;
use crate::memory_manager;
use crate::memory_manager::MemoryManager;
use crate::runtime::RuntimeGlobals;
use async_trait::async_trait;
use camino::Utf8Path;

use dyn_clone::DynClone;
use parking_lot::RwLockWriteGuard;
use parking_lot::RawRwLock;
use parking_lot::lock_api::RawRwLock as _;
use parking_lot::RwLock;
use rspack_sources::BoxSource;

use ustr::Ustr;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::dependency::BoxDependency;
use crate::dependency::DependenciesBlock;
use crate::errors::miette::Result;
use crate::plugin::PluginDriver;

#[derive(Debug)]
pub struct BuildResult {
   
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
pub trait Module: Debug + DependenciesBlock + Send + Sync + DynClone  {
    fn identifier(&self) -> Ustr;
    async fn build(&mut self, build_context: BuildContext, memory_manager: &MemoryManager) -> Result<BuildResult>;
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
    fn need_build(&self) -> bool;
    fn code_generation(
        &self,
        code_generation_context: CodeGenerationContext,
        memory_manager: &MemoryManager,
    ) -> Result<CodeGenerationResult>;
    fn source_types(&self) -> &[SourceType];
}




#[derive(Debug)]
pub struct ReadonlyModule  {
    _lock: RwLock<()>,
    module: Arc<dyn Module>,
}
pub type ModuleId = Ustr;



use parking_lot::RwLockReadGuard;
#[derive(Debug)]
pub struct RwCell<T:?Sized>(Arc<RwLock<T>>);


// new

#[derive(Debug)]
pub struct FreezeCell<T>(RwCell<T>);
impl<T> RwCell<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read()
    }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write()
    }
    pub fn new(value: T) -> RwCell<T> {
        RwCell(Arc::new(RwLock::new(value)))
    }
}
impl<T> Clone for RwCell<T> {
    fn clone(&self) -> RwCell<T> {
        RwCell(Arc::clone(&self.0))
    }
}
pub type WritableModule = RwCell<Box<dyn Module>>;