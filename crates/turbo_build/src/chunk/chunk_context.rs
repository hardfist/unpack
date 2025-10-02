use turbo_rcstr::RcStr;
use turbo_tasks::{ResolvedVc, Upcast, Vc};
use anyhow::Result;
use crate::{chunk::chunk_group::ChunkGroup, ident::AssetIdent, module_graph::ModuleGraph, output::{OutputAssets, OutputAssetsWithReferenced}};



#[turbo_tasks::value_trait]
pub trait ChunkingContext {
    #[turbo_tasks::function]
    fn name(self: Vc<Self>) -> Vc<RcStr>;
     #[turbo_tasks::function]
    fn evaluated_chunk_group(
        self: Vc<Self>,
        ident: Vc<AssetIdent>,
        chunk_group: ChunkGroup,
        module_graph: Vc<ModuleGraph>,
    ) -> Vc<ChunkGroupResult>;
}
pub trait ChunkingContextExt {
    fn evaluated_chunk_group_assets(
        self: Vc<Self>,
        ident: Vc<AssetIdent>,
        chunk_group: ChunkGroup,
        module_graph: Vc<ModuleGraph>,
    ) -> Vc<OutputAssetsWithReferenced>;
}




impl<T: ChunkingContext + Send + Upcast<Box<dyn ChunkingContext>>> ChunkingContextExt for T{
    fn evaluated_chunk_group_assets(
        self: Vc<Self>,
        ident: Vc<AssetIdent>,
        chunk_group: ChunkGroup,
        module_graph: Vc<ModuleGraph>,
    ) -> Vc<OutputAssetsWithReferenced> {
        evaluated_chunk_group_assets(
            Vc::upcast_non_strict(self),
            ident,
            chunk_group,
            module_graph,
        )
    }   
}
#[turbo_tasks::function]
async fn evaluated_chunk_group_assets(
    chunking_context: Vc<Box<dyn ChunkingContext>>,
    ident: Vc<AssetIdent>,
    chunk_group: ChunkGroup,
    module_graph: Vc<ModuleGraph>,
) -> Result<Vc<OutputAssetsWithReferenced>> {
    let evaluated_chunk_group = chunking_context
        .evaluated_chunk_group(ident, chunk_group, module_graph)
        .await?;
    Ok(OutputAssetsWithReferenced {
        assets: evaluated_chunk_group.assets,
        referenced_assets: evaluated_chunk_group.referenced_assets,
    }
    .cell())
}

#[turbo_tasks::value(shared)]
#[derive(Clone, Copy)]
pub struct ChunkGroupResult {
    pub assets: ResolvedVc<OutputAssets>,
    pub referenced_assets: ResolvedVc<OutputAssets>,
}