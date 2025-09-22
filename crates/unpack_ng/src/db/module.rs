
use std::path::PathBuf;

use swc_core::atoms::Atom;
use crate::db::Db;
use crate::db::{ast::collector::ModuleReference};

#[salsa::tracked]
pub struct ESMModule<'db> {
    id: PathBuf,
    module_references: Vec<ModuleReference<'db>>,
}