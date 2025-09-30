use petgraph::adj::NodeIndex;
use rustc_hash::FxHashMap;
use turbo_tasks::{ResolvedVc, Vc};
use anyhow::Result;
use crate::{chunk::chunk_group::{ ChunkGroupEntry}, module::Module};

#[turbo_tasks::value(shared)]
pub struct ModuleGraph {
    pub graphs: Vec<ResolvedVc<SingleModuleGraph>>,
}
#[turbo_tasks::value_impl]
impl ModuleGraph { 
    #[turbo_tasks::function]
    pub async fn from_entries(entries: Vc<GraphEntries>) -> Result<Vc<Self>> {
        
        let single_graph = SingleModuleGraph::new_with_entries(entries).to_resolved().await?;
        let module_graph= ModuleGraph {
            graphs: vec![single_graph]
        };
        Ok(module_graph.cell())
    }
}

#[turbo_tasks::value(cell="new", eq="manual", into="new")]
#[derive(Debug,Clone)]
pub struct SingleModuleGraph {
    modules: FxHashMap<ResolvedVc<Box<dyn Module>>, NodeIndex>,
    entries: GraphEntriesT,
}
pub type GraphEntriesT = Vec<ChunkGroupEntry>;

#[turbo_tasks::value(transparent)]
pub struct GraphEntries(GraphEntriesT);

#[turbo_tasks::value_impl]
impl SingleModuleGraph {
    #[turbo_tasks::function]
    pub async fn new_with_entries(entries: Vc<GraphEntries> ) -> Result<Vc<Self>> {
        SingleModuleGraph::new_inner(&*entries.await?).await
    }
    
}
impl SingleModuleGraph {
    async fn new_inner(entries: &GraphEntriesT) -> Result<Vc<Self>> {
        let graph= Self {
            modules: FxHashMap::default(),
            entries: entries.clone(),
        };
        Ok(graph.cell())
    }
}