use turbo_rcstr::RcStr;
use turbo_tasks::{value_impl, vdbg, FxIndexSet, ReadRef, ResolvedVc, TryFlatJoinIterExt, ValueToString, Vc};
use turbo_tasks_fs::FileSystemPath;

use crate::{chunk::chunk_reference::{ChunkableModuleReference, ChunkingType, ChunkingTypeOption}, file_source::FileSource, module::{EcmascriptModuleAsset, Module, Modules}, module_graph::ExportUsage};
use anyhow::{Result,Ok};

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


#[turbo_tasks::value(shared)]
#[derive(Hash, Debug)]
pub struct EsmAssetReference {
    pub origin: ResolvedVc<FileSystemPath>,
    pub request: String,
}
impl ValueToString for EsmAssetReference {
    fn to_string(self:turbo_tasks::Vc<Self>) -> Vc<RcStr>where Self:Sized {
        todo!()
    }
}
#[turbo_tasks::value_impl]
impl EsmAssetReference {
    #[turbo_tasks::function]
    pub fn new(origin: ResolvedVc<FileSystemPath>, request:String)->Vc<Self> {
        Self {
            origin,
            request
        }.cell()
    }
}

#[turbo_tasks::value_impl]
impl ModuleReference for EsmAssetReference {
    #[turbo_tasks::function]
    async fn resolve_reference(&self) -> Result<Vc<ModuleResolveResult>>{
        let request = self.request.clone();
        let origin = self.origin;
        let full_path = origin.await?.parent().join(&request)?;
        let source = Vc::upcast( FileSource::new(full_path));
        let module = ResolvedVc::upcast(EcmascriptModuleAsset::new(source).to_resolved().await?);
        Ok(ModuleResolveResult {
            modules: vec![module]
        }.cell())
    }
}
#[value_impl]
impl ChunkableModuleReference for EsmAssetReference {
    #[turbo_tasks::function]
    fn chunking_type(self:turbo_tasks::Vc<Self>) -> Result<Vc<ChunkingTypeOption>> {
        let chunk_type: Vc<ChunkingTypeOption> = Vc::cell(Some(ChunkingType::Parallel));
        Ok(chunk_type)
    }
}