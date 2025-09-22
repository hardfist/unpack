pub mod collector;
use anyhow::Result;
use miette::LabeledSpan;
use std::path::PathBuf;
use std::sync::Arc;
use swc_core::atoms::Atom;
use swc_core::common::{FileName, SourceMap, Spanned};
use swc_core::ecma::ast::Program;
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::db::Db;
use crate::db::ast::collector::DependencyCollector;
use crate::db::file::FileSource;
use crate::db::module::ESMModule;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct AST {
    pub program: Program,
}
pub fn parse(db: &dyn Db, file: FileSource) -> miette::Result<ESMModule> {
    let origin_path = file.path(db);
    let content = file.content(db);
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
            return Err(
                miette::miette!(labels = labels, "parse error").with_source_code(content.clone())
            );
        }
    };

    let mut collector = DependencyCollector::new(db);
    program.visit_with(&mut collector);
    let references: Vec<_>= collector.module_references.into_iter().collect();
    
    Ok(ESMModule::new(
        db,
        origin_path,
        references
    ))
}
