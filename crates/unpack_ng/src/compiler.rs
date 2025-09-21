use std::path::PathBuf;

use anyhow::Context;
use notify_debouncer_mini::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use crossbeam_channel::{unbounded, Sender};
use salsa::Setter;

use crate::db::file::SourceFile;
use crate::db::{Db, RootDatabase};

pub fn bundle(db: &dyn Db,entry: SourceFile) -> String{
    let content = entry.content(db);
    content
}
// incremental dev mode
pub fn dev(entry: PathBuf) -> anyhow::Result<()> {
    let (tx, rx) = unbounded();
    let mut db = RootDatabase::new(tx);
    let entry_file = db.add_entry(std::path::PathBuf::from(&entry)).unwrap();
    loop {
        let build_result = bundle(&db,entry_file);
        for log in db.logs.lock().unwrap().drain(..) {
            eprintln!("{log}");
        }
        dbg!( build_result);
        for event in rx.recv()?.unwrap(){
             let path = event.path.canonicalize().with_context(|| {
                format!("Failed to canonicalize path {}", event.path.display())
            })?;
            let file = match db.files.get(&path) {
                Some(file) => *file,
                None => continue,
            };
            let content = std::fs::read_to_string(&path)?;
            file.set_content(&mut db).to(content);
        }
    }
}
// incremental build with persistent cache
pub fn build(){

}