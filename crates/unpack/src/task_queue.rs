use std::collections::VecDeque;

use crate::{dependency::{BoxModuleDependency, DependencyId, ModuleDependency}, module::ModuleId};
#[derive(Debug)]
pub(crate) enum Task {
    Build(BuildTask),
    Factorize(FactorizeTask),
    Add(AddTask),
    ProcessDeps(ProcessDepsTask)
}



#[derive(Debug)]
pub(crate) struct BuildTask {

}
#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency_id: DependencyId,
    pub(crate) origin_module_id: Option<ModuleId>
}
#[derive(Debug)]
pub(crate) struct AddTask {

}
#[derive(Debug)]
pub(crate) struct ProcessDepsTask {

}
#[derive(Debug)]
pub struct TaskQueue {
    queue: VecDeque<Task>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            queue: VecDeque::new(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.queue.push_back(task);
    }
    pub fn get_next_task(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}