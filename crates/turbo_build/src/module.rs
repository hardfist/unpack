use std::io::Read;

use turbo_tasks::{ResolvedVc, TaskInput, Vc};
use turbo_tasks_fs::{rope::Rope, FileSystemPath};
use anyhow::{Context, Result};
use crate::{asset::Asset, asset_content::AssetContent, ident::AssetIdent, reference::{EsmAssetReference, ModuleReference, ModuleReferences}, source::Source};

#[turbo_tasks::value_trait]
pub trait Module: Asset {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent>;
    #[turbo_tasks::function]
    fn references(self: Vc<Self>) -> Result<Vc<ModuleReferences>>;
}

#[turbo_tasks::value]
#[derive(Clone, Debug)]
pub struct EcmascriptModuleAsset {
    pub source: ResolvedVc<Box<dyn Source>>,
}


#[turbo_tasks::value_impl]
impl EcmascriptModuleAsset {
    #[turbo_tasks::function]
    pub fn new(source: ResolvedVc<Box<dyn Source>>) -> Vc<EcmascriptModuleAsset> {
        EcmascriptModuleAsset {
            source
        }.cell()
    }
    #[turbo_tasks::function]
    pub fn origin_path(&self) -> Vc<FileSystemPath> {
        self.source.ident().path()
    }
}

#[turbo_tasks::value_impl]
impl Module for EcmascriptModuleAsset {
    #[turbo_tasks::function]
    fn ident(self:turbo_tasks::Vc<Self>) -> Vc<AssetIdent> {
        todo!()
    }
    #[turbo_tasks::function]
    fn references(self:Vc<Self>) -> Result<Vc<ModuleReferences>>{
        Ok(self.analyze().references())
    }
}
#[turbo_tasks::value_impl]
impl Asset for EcmascriptModuleAsset {
    #[turbo_tasks::function]
    fn content(&self) -> Vc<AssetContent>{
        self.source.content()
    }
}
#[turbo_tasks::value_impl]
impl EcmascriptModuleAsset {
 
    // build_module
    #[turbo_tasks::function]
    async fn module_content_options(self: ResolvedVc<Self>)-> Result<Vc<EcmascriptModuleOptions>>{
       
        Ok(
            EcmascriptModuleOptions {
               references: self.references().to_resolved().await?
            }.cell()
        )
    }
}
impl EcmascriptModuleAsset {
    // codegen
    pub fn module_content(
        self: Vc<Self>,
    ) -> Vc<EcmascriptModuleContent> {
        let module_content_options = self.module_content_options();
        EcmascriptModuleContent::new(module_content_options)
    }
    
}


#[turbo_tasks::function]
pub async fn analyze_ecmascript_module(
    module: ResolvedVc<EcmascriptModuleAsset>
) -> Result<Vc<AnalyzeEcmascriptModuleResult>>{
    let source = module.await?.source;
    let file_content = source.content().await?.content.await?;
    let file = file_content.as_content().expect("failed to read content");
    let content = file.content().to_str()?;
    let requests: Vec<&str> = content.split("\n").collect();
    
    
    let mut references: Vec<ResolvedVc<Box<dyn ModuleReference>>> = vec![];
    for request in requests {
        if request.trim().is_empty() {
            continue;
        }
        let origin_path = module.origin_path();

        let reference: Vc<Box<dyn ModuleReference>> =Vc::upcast(EsmAssetReference::new(origin_path,request.to_string()));
        
        references.push(reference.to_resolved().await?);
    }
    Ok(AnalyzeEcmascriptModuleResult {
        references
    }.cell())
}

#[turbo_tasks::value]
#[derive(Debug,Clone)]
pub struct AnalyzeEcmascriptModuleResult {
    pub references: Vec<ResolvedVc<Box<dyn ModuleReference>>>
}
#[turbo_tasks::value_impl]
impl AnalyzeEcmascriptModuleResult {
    #[turbo_tasks::function]
    pub async fn references(&self) -> Result<Vc<ModuleReferences>> {
        Ok(Vc::cell(self.references.iter().copied().collect()))
    }
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

///  --------------------- Analyzable --------------------
#[turbo_tasks::value_trait]
pub trait EcmascriptModuleAnalyzable: Module + Asset {
    // analyze references
    #[turbo_tasks::function]
    fn analyze(self: Vc<Self>) -> Vc<AnalyzeEcmascriptModuleResult>;
    // build_module
    #[turbo_tasks::function]
    async fn module_content_options(self: ResolvedVc<Self>) -> Result<Vc<EcmascriptModuleOptions>>;
}

#[turbo_tasks::value_impl]
impl EcmascriptModuleAnalyzable for EcmascriptModuleAsset {
    // analyze references
    #[turbo_tasks::function]
    fn analyze(self: Vc<Self>) -> Vc<AnalyzeEcmascriptModuleResult>{
        analyze_ecmascript_module(self)
    }
     // build_module
    #[turbo_tasks::function]
    async fn module_content_options(self: ResolvedVc<Self>)-> Result<Vc<EcmascriptModuleOptions>>{
       
        Ok(
            EcmascriptModuleOptions {
               references: self.references().to_resolved().await?
            }.cell()
        )
    }
    
}

#[turbo_tasks::value(transparent)]
pub struct Modules(Vec<ResolvedVc<Box<dyn Module>>>);