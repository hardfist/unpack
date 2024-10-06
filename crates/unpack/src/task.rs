mod add;
mod build;
mod factorize;
mod process_dependencies;
use std::collections::VecDeque;

pub use add::*;
pub use build::*;
pub use factorize::*;
pub use process_dependencies::*;

#[derive(Debug)]
pub enum Task {
    Add(AddTask),
    Factorize(FactorizeTask),
    ProcessDeps(ProcessDepsTask),
    Build(BuildTask),
}

pub type TaskQueue = VecDeque<Task>;
