use turbo_tasks::{FxIndexSet, ResolvedVc, Vc};
use turbo_tasks_fs::FileSystemPath;
use anyhow::Result;
use crate::asset::Asset;

#[turbo_tasks::value_trait]
pub trait OutputAsset: Asset {
    #[turbo_tasks::function]
    fn path(&self) -> Vc<FileSystemPath>;
    #[turbo_tasks::function]
    fn references(self: Vc<Self>) -> Vc<OutputAssets> {
        OutputAssets::empty()
    }

}


#[turbo_tasks::value(transparent)]
pub struct OutputAssets(Vec<ResolvedVc<Box<dyn OutputAsset>>>);

#[turbo_tasks::value_impl]
impl OutputAssets {
    /// An empty list of [OutputAsset]s
    #[turbo_tasks::function]
    pub fn empty() -> Vc<Self> {
        Vc::cell(Vec::new())
    }
    #[turbo_tasks::function]
    pub async fn concatenate(&self, other: Vc<Self>) -> Result<Vc<Self>> {
        let mut assets: FxIndexSet<_> = self.0.iter().copied().collect();
        assets.extend(other.await?.iter().copied());
        Ok(Vc::cell(assets.into_iter().collect()))
    }
}

#[turbo_tasks::value(shared)]
#[derive(Clone, Copy)]
pub struct OutputAssetsWithReferenced {
    pub assets: ResolvedVc<OutputAssets>,
    pub referenced_assets: ResolvedVc<OutputAssets>,
}

#[turbo_tasks::value_impl]
impl OutputAssetsWithReferenced {
    #[turbo_tasks::function]
    pub fn all_assets(&self) -> Vc<OutputAssets> {
        self.assets.concatenate(*self.referenced_assets)
    }
}
