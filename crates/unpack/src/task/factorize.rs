use camino::Utf8PathBuf;
use miette::Report;

use derive_new::new;
use crate::dependency::BoxModuleDependency;
use crate::module::ModuleId;
// port from https://github.com/webpack/webpack/blob/899f06934391baede59da3dcd35b5ef51c675dbe/lib/Compilation.js#L1842
#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency: BoxModuleDependency,
    pub(crate) origin_module_id: Option<ModuleId>,
    pub(crate) origin_module_context: Option<Utf8PathBuf>
}
#[derive(Debug, new)]
pub(crate) struct FactorizeTaskResult {
    pub(crate) diagnostics: Vec<Report>,
}

