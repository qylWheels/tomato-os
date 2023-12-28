use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::panic;
use crossbeam_queue::ArrayQueue;
use super::*;

pub struct Executor {
    /// Stores task and task id.
    tasks: BTreeMap<u64, Task>,
    /// Only stores task id
    ready_tasks: ArrayQueue<u64>,
}

impl Executor {
    pub fn new() -> Self {
        const MAX_TASK_COUNT: usize = 1024;
        Self {
            tasks: BTreeMap::new(),
            ready_tasks: ArrayQueue::new(MAX_TASK_COUNT),
        }
    }
    
    pub fn spawn(&mut self, task: Task) {
        self.ready_tasks
            .push(task.task_id)
            .expect("Ready task queue is full.");
        let _ = self.tasks
            .insert(task.task_id, task)
            .is_some_and(|_| panic!("Same task ID is not allowed."));
    }

    pub fn run_ready_task(&mut self) {
        while let Some(ready_task_id) = self.ready_tasks.pop() {
            let ready_task = self.tasks.get(&ready_task_id);
            let ready_task = match ready_task {
                Some(ready_task) => ready_task,
                None => panic!("Task(id = {ready_task_id}) not found."),
            };
        }
    }
}

struct TaskWaker {
    task_id: u64,
}

impl TaskWaker {
    fn new(task_id: u64) -> Self {
        Self { task_id }
    }

    fn wake_task(&self, ready_task_queue: &ArrayQueue<u64>) {
        ready_task_queue.push(self.task_id).expect("Ready task queue is full.");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        
    }
}
