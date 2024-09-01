use std::collections::VecDeque;

use derive_new::new;

use crate::errors::UnpackDiagnostic;

use super::BoxTask;

#[derive(Debug)]
pub struct TaskQueue<T> {
    queue: VecDeque<BoxTask<T>>,
}

#[derive(Default,Debug)]
pub(crate) struct MakeArtifact {
    pub(crate) diagnostics: Vec<UnpackDiagnostic>
}
#[derive(new,Debug)]
pub struct MakeTaskContext {
    pub(crate) artifact: MakeArtifact
}

impl<T> TaskQueue<T> {
    pub fn new() -> Self {
        TaskQueue {
            queue: VecDeque::new(),
        }
    }
    pub fn add_task(&mut self, task: BoxTask<T>) {
        self.queue.push_back(task);
    }
    pub fn add_tasks(&mut self, tasks: Vec<BoxTask<T>>) {
        self.queue.extend(tasks)
    }
    pub fn get_next_task(&mut self) -> Option<BoxTask<T>> {
        self.queue.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}