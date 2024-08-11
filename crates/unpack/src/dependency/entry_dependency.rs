use derive_new::new;
use std::path::PathBuf;

use super::{
    module_dependency::{AsModuleDependency, ModuleDependency},
    Dependency,
};

#[derive(new)]
pub struct EntryDependency {
    request: String,
    context: PathBuf,
}

impl Dependency for EntryDependency {}
impl ModuleDependency for EntryDependency {}
impl AsModuleDependency for EntryDependency {}
