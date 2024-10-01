use std::sync::Arc;

use derive_new::new;

use crate::compiler::CompilerOptions;
use crate::dependency::BoxModuleDependency;
use crate::errors::UnpackDiagnostic;
use crate::module::ModuleId;
use crate::normal_module_factory::NormalModuleFactory;

#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency: BoxModuleDependency,
    pub(crate) module_factory: Arc<NormalModuleFactory>,
    pub(crate) origin_module_id: Option<ModuleId>,
    pub(crate) options: Arc<CompilerOptions>,
}
#[derive(Debug, new)]
pub(crate) struct FactorizeTaskResult {
    pub(crate) diagnostics: Vec<UnpackDiagnostic>,
}

