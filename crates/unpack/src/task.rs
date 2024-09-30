mod add;
mod build;
mod factorize;
mod process_dependencies;
pub(crate) use add::*;
pub(crate) use build::*;
pub(crate) use factorize::*;
pub(crate) use process_dependencies::*;
pub(crate) enum Task {
    Add(AddTask),
    Factorize(FactorizeTask),
    ProcessDeps(ProcessDepsTask),
    Build(BuildTask)
}
impl Task  {
    pub(crate) fn run(&mut self){

    }
}