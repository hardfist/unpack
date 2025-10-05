use swc_core::ecma::{
    ast::{CallExpr, Callee, Decl, Expr, Ident, ImportDecl},
    visit::{swc_ecma_ast, Visit, VisitWith},
};

use super::{
    BoxDependency, BoxDependencyTemplate, ConstDependency, EsmExportSpecifierDependency,
    EsmImportSideEffectDependency, SpanExt,
};

// Analyze the AST for all import dependencies
#[derive(Debug, Default)]
pub struct DependencyCollector {
    pub module_dependencies: Vec<BoxDependency>,
    pub presentational_dependencies: Vec<BoxDependencyTemplate>,
}

// utils
impl DependencyCollector {
    fn enter_pattern<F>(&mut self, pattern: &swc_ecma_ast::Pat, on_ident: F)
    where
        F: FnOnce(&mut Self, &Ident) + Copy,
    {
        match pattern {
            swc_ecma_ast::Pat::Ident(ident) => {
                on_ident(self, ident);
            }
            swc_ecma_ast::Pat::Array(array) => {
                for elem in array.elems.iter().flatten() {
                    self.enter_pattern(elem, on_ident);
                }
            }
            swc_ecma_ast::Pat::Object(obj) => {
                for prop in &obj.props {
                    if let swc_ecma_ast::ObjectPatProp::KeyValue(key_value) = prop {
                        self.enter_pattern(&key_value.value, on_ident);
                    }
                }
            }
            _ => {}
        }
    }

    fn enter_declaration<F>(&mut self, decl: &Decl, on_ident: F)
    where
        F: FnOnce(&mut Self, &Ident) + Copy,
    {
        match decl {
            Decl::Class(_c) => {}
            Decl::Fn(_f) => {}
            Decl::Var(var) => {
                for decl in &var.decls {
                    self.enter_pattern(&decl.name, on_ident);
                }
            }
            _ => {}
        }
    }
}

impl DependencyCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Visit for DependencyCollector {
    // Handle static imports
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        let request = import.src.value.clone();

        // Add standard import dependency
        self.module_dependencies
            .push(Box::new(EsmImportSideEffectDependency::new(
                request.clone(),
            )));

        // Add presentational dependency to remove the import later
        self.presentational_dependencies
            .push(Box::new(ConstDependency::new(
                import.span.real_lo(),
                import.span.real_hi(),
                "".into(),
            )));
    }

    fn visit_export_decl(&mut self, decl: &swc_ecma_ast::ExportDecl) {
        self.enter_declaration(&decl.decl, |collector, id| {
            let local_id = &id.sym;
            let name = &id.sym;
            let esm_export_dep =
                EsmExportSpecifierDependency::new(name.clone(), local_id.clone(), decl.span.into());
            collector
                .presentational_dependencies
                .push(Box::new(esm_export_dep));
        });
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
                        .push(Box::new(EsmImportSideEffectDependency::new(request)));
                }
            }
        }

        // Continue traversing inside the call expression
        call.visit_children_with(self);
    }
}
