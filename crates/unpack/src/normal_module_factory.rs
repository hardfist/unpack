use camino::Utf8PathBuf;
use std::sync::Arc;
use crate::{compiler::CompilerOptions, dependency::{BoxDependency, BoxModuleDependency, DependencyId, ModuleDependency}, resolver::UnpackResolver, resolver_factory::ResolverFactory};

#[derive(Debug)]
pub struct NormalModuleFactory {
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>,
    pub resolver_factory: Arc<ResolverFactory>
}
pub struct ModuleFactoryCreateData {
    pub module_dependency: BoxDependency,
    pub context: Utf8PathBuf,
    pub options: Arc<CompilerOptions>
    
}
pub struct ModuleFactoryResult {

}
impl NormalModuleFactory {
    pub fn create(&self,data: ModuleFactoryCreateData){
        let context = data.context;
        let request = data.module_dependency.as_module_dependency().expect("normal module should have module dependency").request();
        let resolve_result = self.resolver_factory.base_resolver.resolve(&context,request);
        dbg!(&resolve_result);
    }
}