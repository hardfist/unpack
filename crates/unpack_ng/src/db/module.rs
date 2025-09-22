
use std::path::PathBuf;

use crate::db::{ast::collector::ModuleReference};


#[salsa::tracked(debug)]
pub struct ESMModule<'db> {
    id: PathBuf,
    pub module_references: Vec<ModuleReference<'db>>,
}