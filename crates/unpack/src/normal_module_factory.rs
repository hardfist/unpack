use camino::Utf8PathBuf;
use std::sync::Arc;
use crate::{compiler::CompilerOptions, dependency::{BoxModuleDependency, DependencyId, ModuleDependency}, resolver::NormalResolver};

pub struct NormalModuleFactory {
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>,
}
pub struct ModuleFactoryCreateData {
    module_dependency: BoxModuleDependency,
    context: Option<Utf8PathBuf>, 
    
}
impl NormalModuleFactory {
    pub fn create(&self,data: ModuleFactoryCreateData){
        let context = data.context.unwrap_or(self.context.clone());
        let request = data.module_dependency.request();
        let resolve_result = NormalResolver::new(self.options.resolve.clone()).resolve(&context, request);
    }
}