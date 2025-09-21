pub mod file;
pub mod ast;
use std::{path::PathBuf, sync::{Arc, Mutex}, time::Duration};
use anyhow::{Result, Ok};
use anyhow::Context;
use crossbeam_channel::Sender;
use dashmap::DashMap;
use notify_debouncer_mini::{new_debouncer, notify::RecommendedWatcher, DebounceEventResult, Debouncer};

use crate::db::file::{SourceFile};

#[salsa::db]
#[derive(Clone)]
pub struct RootDatabase {
    storage:salsa::Storage<Self>,
    pub files: DashMap<PathBuf, SourceFile>,
    pub logs: Arc<Mutex<Vec<String>>>,
    file_watcher: Arc<Mutex<Debouncer<RecommendedWatcher>>>,
}

impl RootDatabase {
    pub fn new(tx: Sender<DebounceEventResult>) -> Self {
        Self {
            storage: Default::default(),
            files:Default::default(),
            file_watcher: Arc::new(Mutex::new(
                new_debouncer(Duration::from_micros(100),tx).unwrap()
            )),
            logs: Default::default()
        }
    }
    pub fn add_entry(&self, path: PathBuf) -> Result<SourceFile> {
        
        let path = path.canonicalize().with_context(|| format!("Failed to canonicalize path: {path:?}"))?;
        
        let result = match self.files.entry(path.clone()) {
            dashmap::mapref::entry::Entry::Occupied(entry) => {
                *entry.get()
            },
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                let content = std::fs::read_to_string(&path)?;
                let watcher = &mut *self.file_watcher.lock().unwrap();
                watcher.watcher().watch(&path, notify_debouncer_mini::notify::RecursiveMode::NonRecursive).unwrap();
                let file = SourceFile::new(self, path.clone(), content);
                entry.insert(file);
                file
            }

        };
        Ok(result)
        
    }
}
#[salsa::db]
pub trait Db: salsa::Database {
    
}
#[salsa::db]
impl Db for RootDatabase {
   
}
impl salsa::Database for RootDatabase {
   
}


#[cfg(test)]
mod tests {
    use crate::db::RootDatabase;

    #[test]
    fn basic_test(){
       
    }
}