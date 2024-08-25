use std::collections::VecDeque;

use super::BoxTask;
use crate::task::Task;

#[derive(Debug)]
pub struct TaskQueue {
    queue: VecDeque<BoxTask>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            queue: VecDeque::new(),
        }
    }
    pub fn add_task(&mut self, task: BoxTask) {
        self.queue.push_back(task);
    }
    pub fn add_tasks(&mut self, tasks: Vec<BoxTask>) {
        self.queue.extend(tasks)
    }
    pub fn get_next_task(&mut self) -> Option<BoxTask> {
        self.queue.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}