pub mod trace_di_graph;
use crate::{
    chunk::{availability_info::{ModuleBatch, RoaringBitmapWrapper}, chunk_group::ChunkGroupEntry, chunk_group_info::{compute_chunk_group_info, ChunkGroupInfo}},
    module::Module,
    module_graph::trace_di_graph::TracedDiGraph,
    reference::{primary_chunkable_referenced_modules, ModuleReference},
};
use anyhow::Result;
use petgraph::{adj::NodeIndex, graph::DiGraph};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use turbo_tasks::{trace::TraceRawVcs, NonLocalValue, ReadRef, ResolvedVc, TaskInput, TryJoinIterExt as _, Vc};

#[turbo_tasks::value(shared)]
#[derive(Debug)]
pub struct ModuleGraph {
    pub graphs: Vec<ResolvedVc<SingleModuleGraph>>,
}
impl ModuleGraph {
     pub async fn read_graphs(self: Vc<ModuleGraph>) -> Result<ModuleGraphRef> {
        Ok(ModuleGraphRef {
            graphs: self.await?.graphs.iter().try_join().await?,
        })
    }
}

pub struct ModuleGraphRef {
    pub graphs: Vec<ReadRef<SingleModuleGraph>>,
}

#[turbo_tasks::value_impl]
impl ModuleGraph {
    #[turbo_tasks::function]
    pub async fn from_entries(entries: Vc<GraphEntries>) -> Result<Vc<Self>> {
        let single_graph = SingleModuleGraph::new_with_entries(entries)
            .to_resolved()
            .await?;
        let module_graph = ModuleGraph {
            graphs: vec![single_graph],
        };
        Ok(module_graph.cell())
    }
    #[turbo_tasks::function]
    pub async fn chunk_group_info(self: Vc<Self>) -> Result<Vc<ChunkGroupInfo>> {
        compute_chunk_group_info(&self.read_graphs().await?).await
    }
}


#[turbo_tasks::value(cell = "new", eq = "manual", into = "new")]
#[derive(Debug, Clone)]
pub struct SingleModuleGraph {
    modules: FxHashMap<ResolvedVc<Box<dyn Module>>, NodeIndex>,
    entries: GraphEntriesT,
    graph: TracedDiGraph<SingleModuleGraphNode, RefData>,
}
pub type GraphEntriesT = Vec<ChunkGroupEntry>;

#[turbo_tasks::value(transparent)]
pub struct GraphEntries(GraphEntriesT);

#[turbo_tasks::value_impl]
impl SingleModuleGraph {
    #[turbo_tasks::function]
    pub async fn new_with_entries(entries: Vc<GraphEntries>) -> Result<Vc<Self>> {
        SingleModuleGraph::new_inner(&*entries.await?).await
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, TraceRawVcs, NonLocalValue)]
pub struct RefData {}
#[derive(Clone, Debug, Serialize, Deserialize, TraceRawVcs, NonLocalValue, PartialEq, Eq, Hash)]
pub struct SingleModuleGraphModuleNode {
    pub module: ResolvedVc<Box<dyn Module>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, TraceRawVcs, NonLocalValue, PartialEq, Eq, Hash)]
pub enum SingleModuleGraphNode {
    Module(SingleModuleGraphModuleNode),
}
#[derive(Clone, Debug, Serialize, Deserialize, TraceRawVcs, NonLocalValue)]
pub struct SingleModuleGraphBuilderEdge {
    to: SingleModuleGraphBuilderNode,
}
#[derive(Clone, Debug, Serialize, Deserialize, TraceRawVcs, NonLocalValue, PartialEq, Eq, Hash)]
pub enum SingleModuleGraphBuilderNode {
    Module { module: ResolvedVc<Box<dyn Module>> },
}
impl SingleModuleGraph {
    async fn new_inner(entries: &GraphEntriesT) -> Result<Vc<Self>> {
        let mut graph: DiGraph<SingleModuleGraphNode, RefData> = DiGraph::new();
        let mut edges: Vec<_> = entries
            .iter()
            .flat_map(|e| e.entries())
            .map(|m| {
                let node = SingleModuleGraphBuilderNode::Module { module: m };

                SingleModuleGraphBuilderEdge { to: node }
            })
            .collect();
        let mut visited_nodes: FxHashSet<SingleModuleGraphBuilderNode> = FxHashSet::default();
        while let Some(edge) = edges.pop() {
            if visited_nodes.contains(&edge.to) {
                continue;
            }
            match edge.to {
                SingleModuleGraphBuilderNode::Module { module } => {
                    let module_graph_module_node = SingleModuleGraphModuleNode { module };
                    let module_graph_node = SingleModuleGraphNode::Module(module_graph_module_node);
                    graph.add_node(module_graph_node);
                    visited_nodes.insert(edge.to.clone());
                    let resolved_module = primary_chunkable_referenced_modules(*module);
                    for (_, _, referenced_modules) in &*resolved_module.await? {
                        for referenced_module in &*referenced_modules {
                            let referenced_node = SingleModuleGraphBuilderNode::Module {
                                module: *referenced_module,
                            };
                            if !visited_nodes.contains(&referenced_node) {
                                edges.push(SingleModuleGraphBuilderEdge {
                                    to: referenced_node,
                                });
                            }
                        }
                    }
                }
            };
        }

        let single_module_graph = SingleModuleGraph {
            modules: FxHashMap::default(),
            entries: entries.clone(),
            graph: TracedDiGraph::new(graph),
        };

        Ok(single_module_graph.cell())
    }
}

impl SingleModuleGraph {
    pub fn graph(&self) -> &TracedDiGraph<SingleModuleGraphNode, RefData> {
        &self.graph
    }

    pub fn entries(&self) -> &GraphEntriesT {
        &self.entries
    }
}

#[turbo_tasks::value]
#[derive(Debug, Clone, Default, Hash)]
pub enum ExportUsage {
    Named(String),
    /// This means the whole content of the module is used.
    #[default]
    All,
    /// Only side effects are used.
    Evaluation,
}


#[turbo_tasks::value]
pub struct ModuleBatchGroup {
    pub items: Vec<ModuleOrBatch>,
    pub chunk_groups: RoaringBitmapWrapper,
}

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
pub enum ModuleOrBatch {
    Module(ResolvedVc<Box<dyn Module>>),
    Batch(ResolvedVc<ModuleBatch>),
    None(usize),
}
