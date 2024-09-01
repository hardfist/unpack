mod add;
mod build;
mod factorize;
mod task_queue;
mod process_dependencies;
pub(crate) use add::*;
pub(crate) use build::*;
pub(crate) use factorize::*;
pub(crate) use task_queue::*;
pub(crate) use process_dependencies::*;
use miette::Result;
use std::{collections::VecDeque, fmt::Debug};
pub trait Task<T>: Debug {
    fn run(self: Box<Self>, context: &mut T) -> TaskResult<T>;
}
pub type BoxTask<Context> = Box<dyn Task<Context>>;
pub type TaskResult<Context> = Result<Vec<BoxTask<Context>>>;