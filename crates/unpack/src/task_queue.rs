use std::collections::VecDeque;

#[derive(Debug)]
pub struct TaskQueue<TaskParams,TaskResult, Executor,>
where
    Executor: Fn(TaskParams) -> TaskResult,
{
    queue: VecDeque<TaskParams>,
    executor: Executor,
}

impl<TaskParams,TaskResult,Executor> TaskQueue<TaskParams,TaskResult,Executor>
where
    Executor: Fn(TaskParams) -> TaskResult,
{
    pub fn new(executor: Executor) -> Self {
        TaskQueue {
            queue: VecDeque::new(),
            executor,
        }
    }

    pub fn add_task(&mut self, task: TaskParams) {
        self.queue.push_back(task);
    }

    pub fn get_next_task(&mut self) -> Option<TaskParams> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn execute_next(&mut self) -> Option<TaskResult> {
        if let Some(task) = self.get_next_task() {
            Some((self.executor)(task))
        }else {
            None
        }
    }
}
#[test]
fn test_executor() {
    let executor = |task: &String| -> String{
        println!("Executing task: {}", task);
        task.clone()
    };

    let mut task_queue = TaskQueue::new(executor);

    task_queue.add_task("Task 1".to_string());
    task_queue.add_task("Task 2".to_string());

    while !task_queue.is_empty() {
       let res =  task_queue.execute_next();
       println!("res:{:?}",res);
    }
}