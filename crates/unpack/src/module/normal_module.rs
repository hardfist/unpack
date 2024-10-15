use std::sync::{mpsc, Arc};

use crate::dependency::{
    AsyncDependenciesBlockId, BoxDependency, BoxDependencyTemplate, ConstDependency,
    DependenciesBlock, DependencyId, HarmonyImportSideEffectDependency, SpanExt,
};
use crate::errors::miette::Result;
use crate::errors::Diagnostics;
use camino::{Utf8Path, Utf8PathBuf};
use miette::{IntoDiagnostic, Report};
use rspack_sources::{BoxSource, OriginalSource, ReplaceSource, SourceExt};
use swc_core::ecma::utils::swc_ecma_ast;

use super::{ast::parse, BuildContext, BuildResult, Module};
use super::{CodeGenerationResult, ModuleGraph};
#[derive(Debug)]
pub struct NormalModule {
    context: Option<Utf8PathBuf>,
    resource_path: Utf8PathBuf,
    request: String,
    diagnostics: Diagnostics,
    original_source: Option<BoxSource>,
    module_dependencies: Vec<DependencyId>,
    presentational_dependencies: Vec<BoxDependencyTemplate>,
    blocks: Vec<AsyncDependenciesBlockId>,
    source: NormalModuleSource,
}
#[derive(Debug, Clone)]
enum NormalModuleSource {
    UnBuild,
    Succeed(BoxSource),
    Failed(Arc<Report>),
}
pub struct CodeGenerationContext<'a> {
    pub module_graph: &'a ModuleGraph,
}
pub struct ParseResult {
    pub(crate) module_dependencies: Vec<BoxDependency>,
    pub(crate) presentational_dependencies: Vec<BoxDependencyTemplate>,
}
impl DependenciesBlock for NormalModule {
    fn add_block_id(&mut self, block_id: AsyncDependenciesBlockId) {
        self.blocks.push(block_id);
    }

    fn get_blocks(&self) -> Vec<AsyncDependenciesBlockId> {
        self.blocks.clone()
    }

    fn add_dependency_id(&mut self, dependency_id: DependencyId) {
        self.module_dependencies.push(dependency_id);
    }

    fn get_dependencies(&self) -> Vec<DependencyId> {
        self.module_dependencies.clone()
    }
}

impl Module for NormalModule {
    fn identifier(&self) -> &str {
        self.resource_path.as_str()
    }
    fn build(&mut self, _build_context: BuildContext) -> Result<BuildResult> {
        let (sender, receiver) = mpsc::channel();
        {
            let resource_path = self.resource_path.clone();

            let _: Result<()> = (|| {
                let content = std::fs::read_to_string(resource_path.clone()).into_diagnostic()?;
                let source =
                    Self::create_source(resource_path.to_string().clone(), content.clone());
                let parse_result = Self::parse(content)?;
                sender.send((source, parse_result)).unwrap();
                Ok(())
            })();
        }
        let (source, parse_result) = receiver.recv().unwrap();
        self.source = NormalModuleSource::Succeed(source.clone());
        Ok(BuildResult {
            module_dependencies: parse_result.module_dependencies,
            presentational_dependencies: parse_result.presentational_dependencies,
        })
    }

    fn get_context(&self) -> Option<&Utf8Path> {
        self.context.as_ref().map(|x| x.as_ref())
    }
    fn code_generation(
        &self,
        code_generation_context: CodeGenerationContext,
    ) -> Result<CodeGenerationResult> {
        let generate_result = match &self.source {
            NormalModuleSource::Failed(_) => {
                todo!("no implemented yet")
            }
            NormalModuleSource::Succeed(source) => {
                self.generate(source.clone(), &code_generation_context)
            }
            NormalModuleSource::UnBuild => {
                panic!("should have source")
            }
        };

        Ok(CodeGenerationResult {
            source: generate_result?,
        })
    }
}
impl NormalModule {
    pub fn new(request: String, resource_path: Utf8PathBuf) -> Self {
        let context = resource_path.parent().map(|x| x.to_owned());
        Self {
            request,
            resource_path,
            diagnostics: vec![],
            original_source: None,
            context,
            blocks: vec![],
            module_dependencies: vec![],
            presentational_dependencies: vec![],
            source: NormalModuleSource::UnBuild,
        }
    }

    fn generate(
        &self,
        source: BoxSource,
        code_generation_context: &CodeGenerationContext,
    ) -> Result<BoxSource> {
        let mut source = ReplaceSource::new(source);
        self.module_dependencies.iter().for_each(|dep_id| {
            if let Some(dependency) = code_generation_context
                .module_graph
                .dependency_by_id(*dep_id)
                .as_dependency_template()
            {
                dependency.apply(&mut source, code_generation_context);
            }
        });
        self.presentational_dependencies
            .iter()
            .for_each(|dependency| {
                dependency.apply(&mut source, code_generation_context);
            });

        Ok(source.boxed())
    }
}

impl NormalModule {
    fn create_source(resource_path: String, content: String) -> BoxSource {
        OriginalSource::new(content, resource_path).boxed()
    }
    fn parse(content: String) -> Result<ParseResult> {
        parse(content)
    }
}
