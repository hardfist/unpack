use crate::compiler::CompilerOptions;
use camino::Utf8PathBuf;
use miette::Result;
use std::{fmt::Debug, sync::Arc};
use async_trait::async_trait;
#[derive(Clone,Debug)]
pub struct PluginContext {
    pub options: Arc<CompilerOptions>,
}

#[derive(Debug,Clone)]
pub struct ResolveArgs {
    pub path: Utf8PathBuf,
    pub context: Utf8PathBuf
}
#[derive(Debug,Clone)]
pub struct LoadArgs {
    pub path: Utf8PathBuf
}
#[async_trait]
pub trait Plugin: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    async fn resolve(&self, _ctx: Arc<PluginContext>, _args: ResolveArgs) -> Result<Option<String>> {
        Ok(None)
    }
    async fn load(&self, _ctx: Arc<PluginContext>,_args: LoadArgs) -> Result<Option<Vec<u8>>>{
        Ok(None)
    }
}

pub type BoxPlugin = Arc<dyn Plugin>;

#[derive(Clone,Debug)]
pub struct PluginDriver {
    pub plugins: Vec<BoxPlugin>,
    pub plugin_context: Arc<PluginContext>
}
impl PluginDriver {
    pub async fn run_resolve_hook(&self,args:ResolveArgs)-> Result<Option<String>>{
        for plugin in &self.plugins {
            let resolve = plugin.resolve(self.plugin_context.clone(), args.clone()).await?;
            if resolve.is_some() {
                return Ok(resolve)
            }else{
                continue;
            }
        }
        Ok(None)
    }
    pub async fn run_load_hook(&self, args: LoadArgs) -> Result<Option<Vec<u8>>> {
         for plugin in &self.plugins {
            let load_result = plugin.load(self.plugin_context.clone(), args.clone()).await?;
            if load_result.is_some() {
                return Ok(load_result)
            }else{
                continue;
            }
        }
        Ok(None)
    }
}