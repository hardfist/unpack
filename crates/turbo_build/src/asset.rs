
use turbo_tasks::{ Vc};


use crate::asset_content::AssetContent;


/// An asset. It also forms a graph when following [Asset::references].
#[turbo_tasks::value_trait]
pub trait Asset {
    /// The content of the [Asset].
    #[turbo_tasks::function]
    fn content(self: Vc<Self>) -> Vc<AssetContent>;
}