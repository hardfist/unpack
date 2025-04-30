use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::ParseResult;
use crate::dependency::{
    BoxDependency, BoxDependencyTemplate, ConstDependency, HarmonyImportSideEffectDependency,
};
use miette::Result;

pub fn parse2(source: String) -> Result<ParseResult> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path("a.jsx").unwrap();

    let ret = Parser::new(&allocator, source.as_ref(), source_type).parse();
    let mut presentational_dependencies: Vec<BoxDependencyTemplate> = vec![];
    let mut module_dependencies: Vec<BoxDependency> = vec![];
    for stmt in ret.program.body {
        if let Statement::ImportDeclaration(import) = stmt {
            let request = import.source.value.to_string();
            module_dependencies.push(Box::new(HarmonyImportSideEffectDependency::new(request.into())));
            presentational_dependencies.push(Box::new(ConstDependency {
                start: import.span.start,
                end: import.span.end,
                content: "".into(),
            }))
        }
    }
    Ok(ParseResult {
        presentational_dependencies,
        module_dependencies,
    })
}
