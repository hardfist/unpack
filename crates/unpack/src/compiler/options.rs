use std::{path::PathBuf};

#[derive(Clone)]
pub struct EntryItem {
    pub name: String,
    pub import: String
}

#[derive(Clone)]
pub struct CompilerOptions {
    pub context: PathBuf,
    pub entry:  EntryItem
}