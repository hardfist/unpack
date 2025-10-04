


use anyhow::anyhow;
use turbo_tasks::Vc;
use turbo_tasks_fs::{FileContent, FileSystemEntryType, FileSystemPath};

use crate::{asset::Asset, asset_content::AssetContent, ident::AssetIdent, source::Source};



#[turbo_tasks::value]
#[derive(Clone, Debug, Hash)]
pub struct FileSource {
    pub path: FileSystemPath
}
#[turbo_tasks::value_impl]
impl FileSource {
    #[turbo_tasks::function]
    pub fn new(path: FileSystemPath) -> Vc<FileSource> {
        FileSource { path }.cell()
    }
}
#[turbo_tasks::value_impl]
impl Source for FileSource {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent>{
        AssetIdent::from_path(self.path.clone())
    }
}
#[turbo_tasks::value_impl]
impl Asset for FileSource {
  
    #[turbo_tasks::function]
    async fn content(&self) -> anyhow::Result<Vc<AssetContent>> {
        let file_type = &*self.path.get_type().await?;
        match file_type {
            FileSystemEntryType::File => {
                Ok(AssetContent::new(self.path.read().to_resolved().await?))
            }
            FileSystemEntryType::NotFound => {
                Ok(AssetContent::new(FileContent::NotFound.resolved_cell()))
            }
            _ => Err(anyhow!("Invalid file type {:?}", file_type)),
        }
    }
}
