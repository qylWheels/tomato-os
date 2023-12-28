mod executor;

extern crate alloc;
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_TASK_ID_ALLOCATOR: AtomicU64 = AtomicU64::new(0);

pub struct Task {
    task_id: u64,
    task: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a task instance.
    pub fn new<T>(task: T) -> Self
    where
        T: Future<Output = ()> + 'static
    {
        Self {
            task_id: GLOBAL_TASK_ID_ALLOCATOR.fetch_add(1, Ordering::Relaxed),
            task: Box::pin(task),
        }
    }
}
