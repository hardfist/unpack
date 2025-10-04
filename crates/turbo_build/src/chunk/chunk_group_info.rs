use std::ops::Deref;

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use turbo_rcstr::RcStr;
use turbo_tasks::{FxIndexSet, ResolvedVc, Vc};
use anyhow::Result;

use crate::{chunk::{availability_info::RoaringBitmapWrapper, chunk_group::ChunkGroup}, module::Module, module_graph::ModuleGraphRef};

pub async fn compute_chunk_group_info(
    graphs: &ModuleGraphRef,
) -> Result<Vc<ChunkGroupInfo>> {
    todo!()
}

#[turbo_tasks::value]
pub struct ChunkGroupInfo {
    pub module_chunk_groups: FxHashMap<ResolvedVc<Box<dyn Module>>, RoaringBitmapWrapper>,
    #[turbo_tasks(trace_ignore)]
    pub chunk_groups: FxIndexSet<ChunkGroup>,
    #[turbo_tasks(trace_ignore)]
    pub chunk_group_keys: FxIndexSet<ChunkGroupKey>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkGroupKey {
    /// e.g. a page
    Entry(Vec<ResolvedVc<Box<dyn Module>>>),
    /// a module with an incoming async edge
    Async(ResolvedVc<Box<dyn Module>>),
    /// a module with an incoming non-merging isolated edge
    Isolated(ResolvedVc<Box<dyn Module>>),
    /// a module with an incoming merging isolated edge
    IsolatedMerged {
        parent: ChunkGroupId,
        merge_tag: RcStr,
    },
    /// a module with an incoming non-merging shared edge
    Shared(ResolvedVc<Box<dyn Module>>),
    /// a module with an incoming merging shared edge
    SharedMerged {
        parent: ChunkGroupId,
        merge_tag: RcStr,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkGroupId(u32);

impl From<usize> for ChunkGroupId {
    fn from(id: usize) -> Self {
        Self(id as u32)
    }
}

impl Deref for ChunkGroupId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}