pub mod file;
pub mod ast;
use std::{path::PathBuf, sync::{Arc, Mutex}, time::Duration};
use anyhow::{Result, Ok};
use anyhow::Context;
use crossbeam_channel::Sender;
use notify_debouncer_mini::{new_debouncer, notify::RecommendedWatcher, DebounceEventResult, Debouncer};

use crate::db::file::Files;

#[salsa::db]
#[derive(Clone)]
pub struct RootDatabase {
    storage:salsa::Storage<Self>,
    files: Files,
    file_watcher: Arc<Mutex<Debouncer<RecommendedWatcher>>>,
}

impl RootDatabase {
    pub fn new(tx: Sender<DebounceEventResult>) -> Self {
        Self {
            storage: Default::default(),
            files: Files::new(),
            file_watcher: Arc::new(Mutex::new(
                new_debouncer(Duration::from_micros(100),tx).unwrap()
            ))
        }
    }
    pub fn add_entry(&self, path: PathBuf) -> Result<()> {
        let path = path.canonicalize().with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
        Ok(())
    }
}
#[salsa::db]
pub trait Db: salsa::Database {
    fn files(&self) -> &Files;
}
impl salsa::Database for RootDatabase {
   
}

#[salsa::db]
impl Db for RootDatabase {
    fn files(&self) -> &Files {
        &self.files
    }
}

#[cfg(test)]
mod tests {
    use crate::db::RootDatabase;

    #[test]
    fn basic_test(){
       
    }
}