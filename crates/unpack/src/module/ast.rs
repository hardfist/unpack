use std::sync::Arc;

use crate::dependency::
    DependencyCollector
;
use crate::errors::miette::{miette, Result};
use miette::LabeledSpan;
use swc_core::common::{FileName, SourceMap, Spanned};
use swc_core::ecma::ast::Program;
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitWith};

use super::ParseResult;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct AST {
    pub program: Program,
}
pub fn parse(content: String) -> Result<ParseResult> {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(
        Arc::new(FileName::Custom("input.js".into())),
        content.clone(),
    );

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
            return Err(miette!(labels = labels, "parse error").with_source_code(content.clone()));
        }
    };

    let mut collector = DependencyCollector::new();
    program.visit_with(&mut collector);
    Ok(ParseResult {
        module_dependencies: collector.module_dependencies,
        presentational_dependencies: collector.presentational_dependencies,
    })
}
