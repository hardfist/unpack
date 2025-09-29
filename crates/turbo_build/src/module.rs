use std::io::Read;

use turbo_tasks::{value_impl, ResolvedVc, TaskInput, Vc};
use turbo_tasks_fs::rope::Rope;
use anyhow::{Context, Result};
use crate::{asset::Asset, asset_content::AssetContent, ident::AssetIdent, reference::{self, ModuleReference, ModuleReferences}, source::Source};

#[turbo_tasks::value_trait]
pub trait Module: Asset {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent>;
    #[turbo_tasks::function]
    fn references(self: Vc<Self>) -> Vc<ModuleReferences>;
}

#[turbo_tasks::value]
#[derive(Clone, Debug)]
pub struct EcmascriptModule {
    pub source: ResolvedVc<Box<dyn Source>>,
}


#[turbo_tasks::value_impl]
impl EcmascriptModule {
    #[turbo_tasks::function]
    pub fn new(source: ResolvedVc<Box<dyn Source>>) -> Vc<EcmascriptModule> {
        EcmascriptModule {
            source
        }.cell()
    }
}
impl Module for EcmascriptModule {
    fn ident(self:turbo_tasks::Vc<Self>) -> Vc<AssetIdent>where Self:Sized {
        todo!()
    }

    fn references(self:turbo_tasks::Vc<Self>) -> Vc<ModuleReferences>where Self:Sized {
        todo!()
    }
}
#[turbo_tasks::value_impl]
impl Asset for EcmascriptModule {
    #[turbo_tasks::function]
    fn content(&self) -> Vc<AssetContent>{
        self.source.content()
    }
}
#[turbo_tasks::value_impl]
impl EcmascriptModule {
    #[turbo_tasks::function]
    async fn references(self: ResolvedVc<Self>) -> Result<Vc<ModuleReferences>> {
        let analyze_result = analyze_ecmascript_module(*self).await?;
        Ok(Vc::cell(analyze_result.references.clone()))
    }   
    #[turbo_tasks::function]
    async fn module_content_options(self: ResolvedVc<Self>)-> Result<Vc<EcmascriptModuleOptions>>{
       
        Ok(
            EcmascriptModuleOptions {
               references: self.references().to_resolved().await?
            }.cell()
        )
    }
}
impl EcmascriptModule {
    fn module_content(
        self: Vc<Self>,
    ) -> Vc<EcmascriptModuleContent> {
        let module_content_options = self.module_content_options();
        EcmascriptModuleContent::new(module_content_options)
    }
}

#[turbo_tasks::function]
pub async fn analyze_ecmascript_module(
    module: ResolvedVc<EcmascriptModule>
) -> Result<Vc<AnalyzeEcmascriptModuleResult>>{
    let source = module.await?.source;
    let file_content = source.content().await?.content.await?;
    let file = file_content.as_content().expect("failed to read content");
    let content = file.content().to_str()?;
    dbg!(content);
    
    let references: Vec<ResolvedVc<Box<dyn ModuleReference>>> = vec![];
    Ok(AnalyzeEcmascriptModuleResult {
        references
    }.cell())
}

#[turbo_tasks::value]
#[derive(Debug,Clone)]
pub struct AnalyzeEcmascriptModuleResult {
    pub references: Vec<ResolvedVc<Box<dyn ModuleReference>>>
}
impl AnalyzeEcmascriptModuleResult {

}

#[turbo_tasks::value(shared)]
pub struct EcmascriptModuleContent {
    pub inner_code: Rope,
}

#[turbo_tasks::value_impl]
impl EcmascriptModuleContent {
    #[turbo_tasks::function]
    pub fn new(options: Vc<EcmascriptModuleOptions>) -> Vc<EcmascriptModuleContent> {
        EcmascriptModuleContent {
            inner_code: Rope::from("console.log('Hello, world!');")
        }.cell()
    }
}

#[turbo_tasks::value(shared)]
#[derive(Clone,Debug,Hash,TaskInput)]
pub struct EcmascriptModuleOptions{
    references: ResolvedVc<ModuleReferences>
}



