mod add;
mod build;
mod factorize;
mod process_dependencies;
pub(crate) use add::*;
pub(crate) use build::*;
pub(crate) use factorize::*;
pub(crate) use process_dependencies::*;
pub(crate) enum Task {
    AddTask(AddTask),
    FactorizeTask(FactorizeTask),
    ProcessDepsTask(ProcessDepsTask),
    BuildTask(BuildTask)
}
impl Task  {
    pub(crate) fn run(&mut self){

    }
}