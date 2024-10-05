use std::sync::Arc;

use miette::LabeledSpan;
use swc_core::common::errors::Handler;
use swc_core::ecma::ast::Program;
use crate::errors::miette::{Result, miette,SourceSpan};
use swc_core::ecma::parser::{Parser, StringInput, Syntax};
use swc_core::common::{FileName, SourceMap, Spanned, DUMMY_SP,};

#[derive(Debug)]
pub struct AST {
    pub(crate) program:Program
}
pub fn parse(content: String) -> Result<AST>{
    let cm = SourceMap::default();
    let fm = cm.new_source_file(Arc::new(FileName::Custom("input.js".into())), content);

    let lexer = swc_core::ecma::parser::lexer::Lexer::new(
        Syntax::Es(Default::default()),
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
            let labels = errors.into_iter().map(|error| {
                let message = error.kind().msg().to_string();
                let span = error.span();
                let start = span.lo.0.saturating_sub(1) as usize;
                let end = span.hi.0.saturating_sub(1) as usize;
                let len = end - start;
                LabeledSpan::new(Some(message), start.into(), len.into())
            }).collect::<Vec<_>>();
            return Err(miette!(labels = labels, "parse error"));
        }
    };

    Ok(AST { program })
}