use derive_new::new;
use camino::Utf8PathBuf;

use super::{
    module_dependency::{AsModuleDependency, ModuleDependency},
    Dependency,
};

// dependency for entry
#[derive(new,Debug)]
pub struct EntryDependency {
    request: String,
    context: Utf8PathBuf,
}

impl Dependency for EntryDependency {}

impl ModuleDependency for EntryDependency {}
impl AsModuleDependency for EntryDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        Some(self)
    }
}
