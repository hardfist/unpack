mod const_dependency;
mod depencency_collector;
mod dependency_block;
mod dependency_id;
mod dependency_range;
mod dependency_template;
mod entry_dependency;
mod esm_export_specifier_dependency;
mod esm_import_side_effect_dependency;
mod init_fragments;
mod module_dependency;
mod swc_span_ext;
use std::fmt::Debug;

use camino::Utf8Path;
pub use const_dependency::*;
pub use depencency_collector::*;
pub use dependency_block::*;
pub use dependency_id::*;
pub use dependency_range::*;
pub use dependency_template::*;
use dyn_clone::{clone_trait_object, DynClone};
pub use entry_dependency::*;
pub use esm_export_specifier_dependency::*;
pub use esm_import_side_effect_dependency::*;
pub use init_fragments::*;
pub use module_dependency::*;
pub use swc_span_ext::*;
pub trait Dependency:
    AsModuleDependency + AsDependencyTemplate + Debug + DynClone + Send + Sync
{
    fn get_context(&self) -> Option<&Utf8Path> {
        None
    }
    fn id(&self) -> DependencyId;
}

clone_trait_object!(Dependency);

pub type BoxDependency = Box<dyn Dependency>;
