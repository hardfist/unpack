use std::mem;
use std::sync::Arc;

use crate::dependency::{
    BoxDependency, BoxDependencyTemplate, ConstDependency, HarmonyImportSideEffectDependency,
    SpanExt,
};
use crate::errors::miette::{miette, Result};
use miette::LabeledSpan;
use swc_core::common::{FileName, SourceMap, Spanned};
use swc_core::ecma::ast::Program;
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::utils::swc_ecma_ast;

use super::ParseResult;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct AST {
    pub program: Program,
}
pub fn parse(content: String) -> Result<ParseResult> {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(Arc::new(FileName::Custom("input.js".into())), content);

    let lexer = swc_core::ecma::parser::lexer::Lexer::new(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(err) => {
            let mut errors = parser.take_errors();
            if errors.is_empty() {
                errors.push(err);
            }
            let labels = errors
                .into_iter()
                .map(|error| {
                    let message = error.kind().msg().to_string();
                    let span = error.span();
                    let start = span.lo.0.saturating_sub(1) as usize;
                    let end = span.hi.0.saturating_sub(1) as usize;
                    let len = end - start;
                    LabeledSpan::new(Some(message), start, len)
                })
                .collect::<Vec<_>>();
            return Err(miette!(labels = labels, "parse error"));
        }
    };
    // Analyze the AST for all import dependencies
    let mut presentational_dependencies: Vec<BoxDependencyTemplate> = vec![];
    let mut module_dependencies: Vec<BoxDependency> = vec![];
    match &program {
        swc_ecma_ast::Program::Module(module) => {
            for item in &module.body {
                if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(
                    import,
                )) = item
                {
                    let request = import.src.value.clone();
                    module_dependencies
                        .push(Box::new(HarmonyImportSideEffectDependency { request }));
                    presentational_dependencies.push(Box::new(ConstDependency::new(
                        import.span.real_lo(),
                        import.span.real_hi(),
                        "".into(),
                    )));
                }
            }
        }
        swc_ecma_ast::Program::Script(_) => {}
    };
    //mem::forget(program);
    Ok(ParseResult {
        module_dependencies,
        presentational_dependencies,
    })
}
