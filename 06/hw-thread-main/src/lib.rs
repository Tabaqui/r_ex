#![deny(unsafe_code)]

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

pub struct ThreadPool {
    _private: (),
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        let _ = (worker_count, task);
        todo!("create shared queue, spawn workers, and return ThreadPool")
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let _ = num;
        todo!("push a task argument into the queue and notify one worker")
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        todo!("set shutdown flag, notify all workers, and join them")
    }
}
