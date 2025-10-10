use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};
use rspack_sources::{BoxSource, ReplaceSource};

use crate::module::CodeGenerationContext;

pub trait DependencyTemplate: Debug + DynClone + Send + Sync {
    fn apply(
        &self,
        _source: &mut ReplaceSource<BoxSource>,
        _code_generation_context: &CodeGenerationContext,
    ) {
    }
}

pub type BoxDependencyTemplate = Box<dyn DependencyTemplate>;

clone_trait_object!(DependencyTemplate);

pub trait AsDependencyTemplate {
    fn as_dependency_template(&self) -> Option<&dyn DependencyTemplate> {
        None
    }
}

impl<T: DependencyTemplate> AsDependencyTemplate for T {
    fn as_dependency_template(&self) -> Option<&dyn DependencyTemplate> {
        Some(self)
    }
}
