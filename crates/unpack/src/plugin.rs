use crate::{compilation::Compilation, compiler::CompilerOptions};
use async_trait::async_trait;
use camino::Utf8PathBuf;
use miette::Result;
use std::{cell::UnsafeCell, fmt::Debug, sync::Arc};
#[derive(Clone, Debug)]
pub struct PluginContext {
    pub options: Arc<CompilerOptions>,
}

#[derive(Debug, Clone)]
pub struct ResolveArgs {
    pub path: Utf8PathBuf,
    pub context: Utf8PathBuf,
}
#[derive(Debug, Clone)]
pub struct LoadArgs {
    pub path: Utf8PathBuf,
}
impl Drop for CompilationCell {
    fn drop(&mut self) {
        println!("compilation_cell dropped")
    }
}
pub struct CompilationCell(UnsafeCell<Compilation>);

impl CompilationCell {
    pub fn new(compilation: Compilation) -> Self {
        Self(UnsafeCell::new(compilation))
    }

    // Safe methods to access the compilation
    pub unsafe fn get(&self) -> *mut Compilation {
        self.0.get()
    }

    pub unsafe fn get_mut(&mut self) -> &mut Compilation {
        self.0.get_mut()
    }
}
unsafe impl Send for CompilationCell {}
unsafe impl Sync for CompilationCell {}

#[async_trait]
pub trait Plugin: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    async fn this_compilation(&self, _ctx: Arc<PluginContext>, _compilation: Arc<CompilationCell>) {
    }
    async fn resolve(
        &self,
        _ctx: Arc<PluginContext>,
        _args: ResolveArgs,
    ) -> Result<Option<String>> {
        Ok(None)
    }
    async fn load(&self, _ctx: Arc<PluginContext>, _args: LoadArgs) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

pub type BoxPlugin = Arc<dyn Plugin>;

#[derive(Clone, Debug)]
pub struct PluginDriver {
    pub plugins: Vec<BoxPlugin>,
    pub plugin_context: Arc<PluginContext>,
}
impl PluginDriver {
    pub async fn run_resolve_hook(&self, args: ResolveArgs) -> Result<Option<String>> {
        for plugin in &self.plugins {
            let resolve = plugin
                .resolve(self.plugin_context.clone(), args.clone())
                .await?;
            if resolve.is_some() {
                return Ok(resolve);
            } else {
                continue;
            }
        }
        Ok(None)
    }
    pub async fn run_load_hook(&self, args: LoadArgs) -> Result<Option<Vec<u8>>> {
        for plugin in &self.plugins {
            let load_result = plugin
                .load(self.plugin_context.clone(), args.clone())
                .await?;
            if load_result.is_some() {
                return Ok(load_result);
            } else {
                continue;
            }
        }
        Ok(None)
    }
    pub async fn run_compilation_hook(&self, compilation: Arc<CompilationCell>) {
        for plugin in &self.plugins {
            plugin
                .this_compilation(self.plugin_context.clone(), compilation.clone())
                .await;
        }
    }
}
