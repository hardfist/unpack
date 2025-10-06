use camino::Utf8PathBuf;
use miette::Report;

use crate::dependency::BoxDependency;
use crate::module::ModuleId;
use derive_new::new;
// port from https://github.com/webpack/webpack/blob/899f06934391baede59da3dcd35b5ef51c675dbe/lib/Compilation.js#L1842
#[derive(Debug)]
pub struct FactorizeTask {
    pub dependencies: Vec<BoxDependency>,
    pub origin_module_id: Option<ModuleId>,
    pub origin_module_context: Option<Utf8PathBuf>,
}
#[derive(Debug, new)]
pub struct FactorizeTaskResult {
    pub diagnostics: Vec<Report>,
}
