use crate::{
    compiler::CompilerOptions,
    dependency::BoxDependency,
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
    pub module_dependency: BoxDependency,
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>,
}
#[derive(Debug)]
pub struct ModuleFactoryResult {
    pub module: NormalModule,
}
impl NormalModuleFactory {
    pub fn create(&self, data: ModuleFactoryCreateData) -> Result<ModuleFactoryResult> {
        let dependency = data.module_dependency.as_module_dependency().unwrap();
        let context = data.context.clone();
        let request = dependency.request();
        let resolve_result = self
            .resolver_factory
            .base_resolver
            .resolve(&context, request)
            .into_diagnostic()?;
        let resource_path = resolve_result.path;
        let module = NormalModule::new(request.to_string(), resource_path);
        Ok(ModuleFactoryResult { module })
    }
}
