
use std::path::PathBuf;

use swc_core::atoms::Atom;

use crate::db::{ast::collector::ModuleReference, Db};

#[salsa::tracked]
pub struct ESMModule<'db> {
    id: PathBuf,
    module_references: Vec<ModuleReference<'db>>,
}

#[salsa::tracked]
impl<'db> ESMModule<'db> {
    pub fn resolve_reference(&self, db: &'db dyn Db, name: Atom) -> Option<ESMModule> {
        // db.lookup_module(name)
        None
    }
}