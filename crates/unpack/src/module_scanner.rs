use std::{path::PathBuf, sync::Arc};

use derive_new::new;

use crate::{compiler::CompilerOptions, dependency::{BoxDependency, EntryDependency}, module::{ModuleId, NormalModule}, task_queue::TaskQueue};

#[derive(new)]
pub struct ModuleScanner {
    options: Arc<CompilerOptions>
}
impl ModuleScanner {
    // add_entry
    pub fn add_entry(&mut self) {
        let entry_dep = EntryDependency::new(
            self.options.entry.import.clone(),
            self.options.context.clone(),
        );
        let add_queue = TaskQueue::new(|s:String| {
            s
        });
        
        loop {
            
        }
       
    }
}
