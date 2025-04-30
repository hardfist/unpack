use std::sync::Arc;

use crate::dependency::{
    BoxDependency, BoxDependencyTemplate, ConstDependency, HarmonyImportSideEffectDependency,
    SpanExt,
};
use crate::errors::miette::{miette, Result};
use miette::LabeledSpan;
use swc_core::common::{FileName, SourceMap, Spanned};
use swc_core::ecma::ast::{CallExpr, Callee, Expr, ImportDecl, Program};
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::utils::swc_ecma_ast;
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
    // Analyze the AST for all import dependencies

    struct DependencyCollector {
        module_dependencies: Vec<BoxDependency>,
        presentational_dependencies: Vec<BoxDependencyTemplate>,
    }

    impl DependencyCollector {
        fn new() -> Self {
            Self {
                module_dependencies: vec![],
                presentational_dependencies: vec![],
            }
        }
    }
    impl Visit for DependencyCollector {
        // Handle static imports
        fn visit_import_decl(&mut self, import: &ImportDecl) {
            let request = import.src.value.clone();

            // Add standard import dependency
            self.module_dependencies
                .push(Box::new(HarmonyImportSideEffectDependency::new(request.clone())));

            // Add presentational dependency to remove the import later
            self.presentational_dependencies
                .push(Box::new(ConstDependency::new(
                    import.span.real_lo(),
                    import.span.real_hi(),
                    "".into(),
                )));
        }

        // Handle dynamic imports: import("./module")
        fn visit_call_expr(&mut self, call: &CallExpr) {
            // Check if this is a dynamic import
            if let Callee::Import(_expr) = &call.callee {
                // Try to extract the string literal for the import path
                if let Some(arg) = call.args.first() {
                    if let Expr::Lit(swc_ecma_ast::Lit::Str(str_lit)) = &*arg.expr {
                        let request = str_lit.value.clone();
                        // Add dynamic import dependency with different type
                        self.module_dependencies
                            .push(Box::new(HarmonyImportSideEffectDependency::new(request)));
                    }
                }
            }

            // Continue traversing inside the call expression
            call.visit_children_with(self);
        }
    }

    let mut collector = DependencyCollector::new();
    program.visit_with(&mut collector);
    Ok(ParseResult {
        module_dependencies: collector.module_dependencies,
        presentational_dependencies: collector.presentational_dependencies,
    })
}
