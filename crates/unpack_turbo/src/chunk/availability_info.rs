use std::ops::{Deref, DerefMut};
use std::hash::Hash;
use roaring::RoaringBitmap;
use serde::{Deserialize, Serialize};

use turbo_tasks::{debug::ValueDebugFormat, trace::TraceRawVcs, turbo_tasks, FxIndexSet, NonLocalValue, ResolvedVc, TaskInput, Vc};

use crate::{asset::Asset, chunk::{chunk_context::ChunkingContext, chunk_item::ChunkItem}, module::Module, module_graph::ModuleGraph};

pub enum AvailabilityInfo {
    /// Availability of modules is not tracked
    Untracked,
    /// Availability of modules is tracked, but no modules are available
    Root,
    /// There are modules already available.
    Complete {
        available_modules: ResolvedVc<AvailableModules>,
    },
}

/// Allows to gather information about which assets are already available.
/// Adding more roots will form a linked list like structure to allow caching
/// `include` queries.
#[turbo_tasks::value]
pub struct AvailableModules {
    parent: Option<ResolvedVc<AvailableModules>>,
    modules: ResolvedVc<AvailableModulesSet>,
}

#[turbo_tasks::value(transparent)]
#[derive(Debug, Clone)]
pub struct AvailableModulesSet(FxIndexSet<ChunkableModuleOrBatch>);


#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    TraceRawVcs,
    NonLocalValue,
    TaskInput,
)]
pub enum ChunkableModuleOrBatch {
    Module(ResolvedVc<Box<dyn ChunkableModule>>),
    Batch(ResolvedVc<ModuleBatch>),
    None(usize),
}

/// A [Module] that can be converted into a [Chunk].
#[turbo_tasks::value_trait]
pub trait ChunkableModule: Module + Asset {
    #[turbo_tasks::function]
    fn as_chunk_item(
        self: Vc<Self>,
        module_graph: Vc<ModuleGraph>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Vc<Box<dyn ChunkItem>>;
}


#[turbo_tasks::value]
pub struct ModuleBatch {
    pub modules: Vec<ResolvedVc<Box<dyn ChunkableModule>>>,
    pub chunk_groups: Option<RoaringBitmapWrapper>,
}

#[derive(
    Clone, Debug, Default, PartialEq, Serialize, Deserialize, TraceRawVcs, ValueDebugFormat,
)]
#[repr(transparent)]
pub struct RoaringBitmapWrapper(#[turbo_tasks(trace_ignore)] pub RoaringBitmap);


impl TaskInput for RoaringBitmapWrapper {
    fn is_transient(&self) -> bool {
        false
    }
}
impl RoaringBitmapWrapper {
    /// Whether `self` contains bits that are not in `other`
    ///
    /// The existing `is_superset` method also returns true for equal sets
    pub fn is_proper_superset(&self, other: &Self) -> bool {
        !self.is_subset(other)
    }

    pub fn into_inner(self) -> RoaringBitmap {
        self.0
    }
}
unsafe impl NonLocalValue for RoaringBitmapWrapper {}

impl Eq for RoaringBitmapWrapper {}
impl Deref for RoaringBitmapWrapper {
    type Target = RoaringBitmap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RoaringBitmapWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Hash for RoaringBitmapWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        struct HasherWriter<'a, H: std::hash::Hasher>(&'a mut H);
        impl<H: std::hash::Hasher> std::io::Write for HasherWriter<'_, H> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.0.write(buf);
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        self.0.serialize_into(HasherWriter(state)).unwrap();
    }
}