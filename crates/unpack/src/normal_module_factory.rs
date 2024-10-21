use crate::{
    compiler::CompilerOptions,
    dependency::BoxDependency,
    module::NormalModule,
    plugin::{LoadArgs, PluginDriver},
    resolver_factory::ResolverFactory,
};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use std::{str::FromStr, sync::Arc};

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
    pub fn create(
        &self,
        data: ModuleFactoryCreateData,
        plugin_driver: PluginDriver,
    ) -> Result<ModuleFactoryResult> {
        let dependency = data.module_dependency.as_module_dependency().unwrap();
        let context = data.context.clone();
        let request = dependency.request();
        let load_result = plugin_driver.run_load_hook(LoadArgs {
            context: context.clone(),
            path: Utf8PathBuf::from_str(request).unwrap(),
        })?;
        let resource_path = match load_result {
            Some(res) => Utf8PathBuf::from(res),
            None => {
                let resolve_result = self
                    .resolver_factory
                    .base_resolver
                    .resolve(&context, request)
                    .into_diagnostic()?;
                let resource_path = resolve_result.path;
                resource_path
            }
        };

        let module = NormalModule::new(request.to_string(), resource_path);
        Ok(ModuleFactoryResult { module })
    }
}
