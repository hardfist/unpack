use std::{convert::TryFrom, ops::Deref};

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use turbo_rcstr::RcStr;
use turbo_tasks::{FxIndexSet, ResolvedVc, Vc};
use anyhow::{anyhow, Result};

use crate::{
    chunk::{availability_info::RoaringBitmapWrapper, chunk_group::{ChunkGroup, ChunkGroupEntry}},
    module::Module,
    module_graph::{ModuleGraphRef, SingleModuleGraphNode},
};

// compute chunk_group from module_graph
pub async fn compute_chunk_group_info(
    graphs: &ModuleGraphRef,
) -> Result<Vc<ChunkGroupInfo>> {
    let mut module_chunk_groups: FxHashMap<ResolvedVc<Box<dyn Module>>, RoaringBitmapWrapper> =
        FxHashMap::default();
    let mut chunk_groups: FxIndexSet<ChunkGroup> = FxIndexSet::default();
    let mut chunk_group_keys: FxIndexSet<ChunkGroupKey> = FxIndexSet::default();

    for graph in &graphs.graphs {
        for node in graph.graph().node_weights() {
            match node {
                SingleModuleGraphNode::Module(module_node) => {
                    module_chunk_groups
                        .entry(module_node.module)
                        .or_insert_with(RoaringBitmapWrapper::default);
                }
            }
        }

        for entry in graph.entries() {
            let (modules, chunk_group, chunk_group_key) = match entry {
                ChunkGroupEntry::Entry(modules) => {
                    let modules_vec = modules.clone();
                    (
                        modules_vec.clone(),
                        ChunkGroup::Entry(modules_vec.clone()),
                        ChunkGroupKey::Entry(modules_vec),
                    )
                }
                ChunkGroupEntry::Async(module) => {
                    let module_resolved = module.clone();
                    let modules_vec = vec![module_resolved.clone()];
                    (
                        modules_vec,
                        ChunkGroup::Async(module_resolved.clone()),
                        ChunkGroupKey::Async(module_resolved),
                    )
                }
            };

            chunk_groups.insert(chunk_group.clone());
            let chunk_group_index = chunk_groups
                .get_index_of(&chunk_group)
                .expect("chunk group should exist after insertion");

            chunk_group_keys.insert(chunk_group_key);

            let chunk_group_index = u32::try_from(chunk_group_index)
                .map_err(|_| anyhow!("chunk group index exceeds u32 range"))?;

            for module in modules {
                let bitmap = module_chunk_groups
                    .entry(module)
                    .or_insert_with(RoaringBitmapWrapper::default);
                bitmap.insert(chunk_group_index);
            }
        }
    }

    Ok(ChunkGroupInfo {
        module_chunk_groups,
        chunk_groups,
        chunk_group_keys,
    }
    .cell())
}

#[derive(Debug,Clone)]
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
