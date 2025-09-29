use turbo_tasks::{ResolvedVc, ValueToString, Vc};

use crate::module::Module;


#[turbo_tasks::value_trait]
pub trait ModuleReference: ValueToString {
    #[turbo_tasks::function]
    fn resolve_reference(self: Vc<Self>) ->Vc<dyn Module>;
}

#[turbo_tasks::value(transparent)]
#[derive(Debug)]
pub struct ModuleReferences(Vec<ResolvedVc<Box<dyn ModuleReference>>>);

#[turbo_tasks::value_impl]
impl ModuleReferences {
    /// An empty list of [ModuleReference]s
    #[turbo_tasks::function]
    pub fn empty() -> Vc<Self> {
        Vc::cell(Vec::new())
    }
}