use derive_new::new;
use camino::Utf8PathBuf;

use super::{
    module_dependency::{AsModuleDependency, ModuleDependency},
    Dependency,
};

// dependency for entry
#[derive(new,Debug,Clone)]
pub struct EntryDependency {
    request: String,
    context: Utf8PathBuf,
}

impl Dependency for EntryDependency {
    fn get_context(&self) -> Option<&camino::Utf8Path> {
        Some(&self.context)
    }
}

impl ModuleDependency for EntryDependency {
    fn request(&self) -> &str {
        &self.request
    }
}
impl AsModuleDependency for EntryDependency {
    fn as_module_dependency(&self) -> Option<&dyn ModuleDependency> {
        Some(self)
    }
    
}
