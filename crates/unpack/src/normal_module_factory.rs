use crate::{
    compiler::CompilerOptions,
    dependency::BoxDependency,
    errors::UnpackDiagnostic,
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
    pub diagnostics: Vec<UnpackDiagnostic>,
}
#[derive(Debug)]
pub struct ModuleFactoryResult {
    full_path: Utf8PathBuf,
    module: NormalModule,
}
impl NormalModuleFactory {
    pub fn create(&self, data: &mut ModuleFactoryCreateData) -> Result<ModuleFactoryResult> {
        let context = data.context.clone();
        let request = data
            .module_dependency
            .as_module_dependency()
            .expect("normal module should have module dependency")
            .request();
        dbg!(&context, &request);
        let resolve_result = self
            .resolver_factory
            .base_resolver
            .resolve(&context, request)
            .into_diagnostic()?;
        let module = NormalModule::new(resolve_result.path.to_string());
        Ok(ModuleFactoryResult {
            full_path: resolve_result.path,
            module,
        })
    }
}
