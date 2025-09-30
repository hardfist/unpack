use turbo_tasks::{FxIndexSet, ReadRef, ResolvedVc, TryFlatJoinIterExt, ValueToString, Vc};

use crate::{chunk::chunk_reference::{ChunkableModuleReference, ChunkingType}, module::{Module, Modules}, module_graph::ExportUsage};
use anyhow::Result;

#[turbo_tasks::value_trait]
pub trait ModuleReference: ValueToString {
    #[turbo_tasks::function]
    fn resolve_reference(self: Vc<Self>) ->Vc<ModuleResolveResult>;
}
#[turbo_tasks::value(shared)]
#[derive(Clone)]
struct ModuleResolveResult {
    pub modules:Vec<ResolvedVc<Box<dyn Module>>>,
}

#[turbo_tasks::value_impl]
impl ModuleResolveResult {
    #[turbo_tasks::function]
    pub fn primary_modules(&self) -> Result<Vc<Modules>> {
       let mut set = FxIndexSet::default();
       for item in self.modules.iter(){
          set.insert(*item);
       }
       Ok(Vc::cell(set.into_iter().collect()))
    }
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

#[turbo_tasks::value(transparent)]
pub struct ModulesWithRefData(Vec<(ChunkingType, ExportUsage, ReadRef<Modules>)>);

#[turbo_tasks::function]
pub async fn primary_chunkable_referenced_modules(
    module: ResolvedVc<Box<dyn Module>>,
) -> Result<Vc<ModulesWithRefData>>{
    let modules = module.references().await?.iter().map(|reference|  async {
        if let Some(chunk_module_reference) = ResolvedVc::try_downcast::<Box<dyn ChunkableModuleReference>>(*reference)
        && let Some(chunking_type) = &*chunk_module_reference.chunking_type().await?
         {
            let resolved = chunk_module_reference.resolve_reference().resolve().await?.primary_modules().await?;
            return Ok(Some(( chunking_type.clone(),ExportUsage::All,resolved)))
        }
        Ok(None)
    }).try_flat_join().await?;
    Ok(Vc::cell(modules))
}