use turbo_tasks::Vc;

use crate::{asset::Asset, ident::AssetIdent};

#[turbo_tasks::value_trait]
pub trait Source: Asset {
    /// The identifier of the [Source]. It's expected to be unique and capture
    /// all properties of the [Source].
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent>;
}
