use turbo_tasks::{ResolvedVc, ValueToString, Vc};

use crate::module::Module;


#[turbo_tasks::value_trait]
pub trait ModuleReference: ValueToString {
    #[turbo_tasks::function]
    fn resolve_reference(self: Vc<Self>) ->Vc<dyn Module>;
}

pub struct ModuleReferences(Vec<ResolvedVc<Box<dyn ModuleReference>>>);