use crate::{
    compiler::CompilerOptions,
    dependency::BoxDependency,
    module::NormalModule,
    plugin::{PluginDriver, ResolveArgs},
    resolver_factory::ResolverFactory,
};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use std::{str::FromStr, sync::Arc};
use tokio::task::spawn_blocking;

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
    pub async fn create(
        &self,
        data: ModuleFactoryCreateData,
        plugin_driver: Arc<PluginDriver>,
    ) -> Result<ModuleFactoryResult> {
        let dependency = data.module_dependency.as_module_dependency().unwrap();
        let context = data.context.clone();
        let request = dependency.request();
        let load_result = plugin_driver
            .run_resolve_hook(ResolveArgs {
                context: context.clone(),
                path: Utf8PathBuf::from_str(request).unwrap(),
            })
            .await?;
        let resource_path = match load_result {
            Some(res) => Utf8PathBuf::from(res),
            None => {
                let resolver_factory = self.resolver_factory.clone();
                let context = context.clone();
                let request = request.to_string();
                let result = spawn_blocking(move || {
                    resolver_factory
                        .base_resolver
                        .resolve(&context, &request)
                })
                .await
                .unwrap();
                result.into_diagnostic()?.path
            }
        };

        let module = NormalModule::new(request.to_string(), resource_path);
        Ok(ModuleFactoryResult { module })
    }
}
