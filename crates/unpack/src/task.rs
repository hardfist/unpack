mod add;
mod build;
mod factorize;
mod process_dependencies;
use std::collections::VecDeque;

pub(crate) use add::*;
pub(crate) use build::*;
pub(crate) use factorize::*;
pub(crate) use process_dependencies::*;

#[derive(Debug)]
pub(crate) enum Task {
    Add(AddTask),
    Factorize(FactorizeTask),
    ProcessDeps(ProcessDepsTask),
    Build(BuildTask)
}

pub type TaskQueue = VecDeque<Task>;