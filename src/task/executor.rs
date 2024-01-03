use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::panic;
use core::task::{Poll, Waker, Context};
use crossbeam_queue::ArrayQueue;
use super::*;

pub struct Executor {
    /// Stores task and task id.
    tasks: BTreeMap<u64, Task>,
    /// Only stores task id.
    ready_tasks: Arc<ArrayQueue<u64>>,
    /// Waker cache.
    waker_cache: BTreeMap<u64, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        const MAX_TASK_COUNT: usize = 1024;
        Self {
            tasks: BTreeMap::new(),
            ready_tasks: Arc::new(ArrayQueue::new(MAX_TASK_COUNT)),
            waker_cache: BTreeMap::new(),
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

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_task();
            self.sleep_if_idle();
        }
    }

    pub fn run_ready_task(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            ready_tasks,
            waker_cache,
        } = self;

        while let Some(task_id) = ready_tasks.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, Arc::clone(&ready_tasks)));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};

        interrupts::disable();
        if self.ready_tasks.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

struct TaskWaker {
    task_id: u64,
    ready_tasks: Arc<ArrayQueue<u64>>,
}

impl TaskWaker {
    fn new(task_id: u64, ready_tasks: Arc<ArrayQueue<u64>>) -> Waker {
        Waker::from(Arc::new({TaskWaker {
            task_id, ready_tasks,
        }}))
    }

    fn wake_task(&self) {
        self.ready_tasks.push(self.task_id).expect("Ready task queue is full.");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
