use camino::Utf8PathBuf;

use super::{module_dependency::ModuleDependency, AsDependencyTemplate, Dependency, DependencyId};

// dependency for entry
#[derive(Debug, Clone)]
pub struct EntryDependency {
    request: String,
    context: Utf8PathBuf,
    pub(crate) id: DependencyId,
}
impl EntryDependency {
    pub fn new(request: String, context: Utf8PathBuf) -> Self {
        Self {
            request,
            context,
            id: DependencyId::new(),
        }
    }
}

impl Dependency for EntryDependency {
    fn get_context(&self) -> Option<&camino::Utf8Path> {
        Some(&self.context)
    }
    fn id(&self) -> DependencyId {
        self.id
    }
}

impl ModuleDependency for EntryDependency {
    fn context(&self) -> &str {
        ""
    }
    fn resource_identifier(&self) -> String {
        format!("{}-{}", self.context(), self.request)
    }
    fn request(&self) -> &str {
        &self.request
    }
}
impl AsDependencyTemplate for EntryDependency {}
