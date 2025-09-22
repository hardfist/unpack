use swc_core::{atoms::Atom, ecma::{
    ast::{CallExpr, Callee, Decl, Expr, Ident, ImportDecl},
    visit::{swc_ecma_ast, Visit, VisitWith},
}};

use crate::db::Db;
#[salsa::interned]
pub struct ModuleReference {
    pub request: Atom,
}

// Analyze the AST for all import dependencies
pub struct DependencyCollector<'db> {
    db: &'db dyn Db,
    pub module_references: Vec<ModuleReference<'db>>
}

// utils
impl<'db> DependencyCollector<'db> {
    fn enter_pattern<F>(&mut self, pattern: &swc_ecma_ast::Pat, on_ident: F)
    where
        F: FnOnce(&mut Self, &Ident) + Copy,
    {
        match pattern {
            swc_ecma_ast::Pat::Ident(ident) => {
                on_ident(self, ident);
            }
            swc_ecma_ast::Pat::Array(array) => {
                for elem in &array.elems {
                    if let Some(elem) = elem {
                        self.enter_pattern(&elem, on_ident);
                    }
                }
            }
            swc_ecma_ast::Pat::Object(obj) => {
                for prop in &obj.props {
                    match prop {
                        swc_ecma_ast::ObjectPatProp::KeyValue(key_value) => {
                            self.enter_pattern(&key_value.value, on_ident);
                        }
                        _ => {}
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
            Decl::Class(c) => {}
            Decl::Fn(f) => {}
            Decl::Var(var) => {
                for decl in &var.decls {
                    self.enter_pattern(&decl.name, on_ident);
                }
            }
            _ => {}
        }
    }
}

impl<'db> DependencyCollector<'db> {
    pub fn new(db: &'db dyn Db) -> Self {
        Self {
            module_references: vec![],
            db
        }
    }
}
impl<'db> Visit for DependencyCollector<'db> {
    // Handle static imports
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        let request = import.src.value.clone();

        // Add standard import dependency
        self.module_references
            .push(ModuleReference::new(
                self.db,
                request.clone(),
            ));

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
                    self.module_references
                        .push(ModuleReference::new(self.db,request.clone()));
                }
            }
        }

        // Continue traversing inside the call expression
        call.visit_children_with(self);
    }
}
