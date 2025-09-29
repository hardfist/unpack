
use anyhow::{anyhow, Result};
use turbo_tasks::{ResolvedVc, Vc};
use turbo_tasks_fs::{FileContent, FileSystemEntryType, FileSystemPath};

use crate::{asset::Asset, asset_content::AssetContent};



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
impl Asset for FileSource {
  
    #[turbo_tasks::function]
    async fn content(&self) -> anyhow::Result<Vc<AssetContent>> {
        let file_type = &*self.path.get_type().await?;
        match file_type {
            FileSystemEntryType::File => {
                Ok(AssetContent::file(self.path.read().to_resolved().await?))
            }
            FileSystemEntryType::NotFound => {
                Ok(AssetContent::file(FileContent::NotFound.resolved_cell()))
            }
            _ => Err(anyhow!("Invalid file type {:?}", file_type)),
        }
    }
}
