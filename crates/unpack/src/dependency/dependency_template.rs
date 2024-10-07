use std::fmt::Debug;

use dyn_clone::DynClone;
use rspack_sources::{BoxSource, ReplaceSource};

use crate::module::CodeGenerationContext;

pub trait DependencyTemplate: Debug + DynClone + Send + Sync {
    fn apply(&self, source:&mut ReplaceSource<BoxSource>, code_generation_context: &CodeGenerationContext ) {

    }
}

pub type BoxDependencyTemplate = Box<dyn DependencyTemplate>;

pub trait AsDependencyTemplate {
    fn as_dependency_template(&self) -> Option<&dyn DependencyTemplate> {
        None
    }
}

impl <T: DependencyTemplate> AsDependencyTemplate for T {
    fn as_dependency_template(&self) -> Option<&dyn DependencyTemplate> {
        Some(self)
    }
}