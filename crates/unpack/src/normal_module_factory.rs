use crate::{
    compiler::CompilerOptions,
    dependency::BoxModuleDependency,
    module::NormalModule,
    resolver_factory::ResolverFactory,
};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use std::sync::Arc;

#[derive(Debug)]
pub struct NormalModuleFactory {
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>,
    pub resolver_factory: Arc<ResolverFactory>,
}
#[derive(Debug)]
pub struct ModuleFactoryCreateData {
    pub module_dependency: BoxModuleDependency,
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>
}
#[derive(Debug)]
pub struct ModuleFactoryResult {
   pub(crate) module: NormalModule,
}
impl NormalModuleFactory {
    pub fn create(&self, data: ModuleFactoryCreateData) -> Result<ModuleFactoryResult> {
        let context = data.context.clone();
        let request = data
            .module_dependency
            .request();
        let resolve_result = self
            .resolver_factory
            .base_resolver
            .resolve(&context, request)
            .into_diagnostic()?;
        let module = NormalModule::new(resolve_result.path.to_string());
        Ok(ModuleFactoryResult {
            module,
        })
    }
}
