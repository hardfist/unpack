use std::path::PathBuf;

use notify_debouncer_mini::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use crossbeam_channel::{unbounded, Sender};

use crate::db::{Db, RootDatabase};

pub fn bundle(){

}
// incremental dev mode
pub fn dev(entry: PathBuf) -> anyhow::Result<()> {
    let (tx, rx) = unbounded();
    let mut db = RootDatabase::new(tx);
    
    let initial = db.add_entry(std::path::PathBuf::from(&entry)).unwrap();
    loop {
        let build_result = bundle();
        
        for event in rx.recv()?.unwrap(){
            dbg!(event);
        }
    }
}
// incremental build with persistent cache
pub fn build(){

}