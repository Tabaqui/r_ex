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

use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

pub type Task = fn(i64);

pub struct ThreadPool {
    handles: Vec<JoinHandle<()>>,
    shared: Arc<Shared>,
}

impl ThreadPool {
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        if worker_count == 0 {
            panic!("You are so bad. Bad to the bones!..\nCount needs to be more than zero.");
        }

        let shared = Arc::new(Shared::new(15));
        let handles: Vec<JoinHandle<()>> = (0..worker_count)
            .map(|_| {
                let local_shared = Arc::clone(&shared);
                thread::spawn(move || {
                    do_task(local_shared, task);
                })
            })
            .collect();

        Self { handles, shared }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().expect("u'd be ok");
        state.queue.push(num);
        drop(state);
        self.shared.has_work.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        let mut state = self.shared.state.lock().expect("u'd be ok");
        state.shutting_down = true;
        drop(state);
        self.shared.has_work.notify_all();
        self.handles.into_iter().for_each(|h| h.join().expect("u'd be ok"));
    }
}

//
//
struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}
impl Shared {
    fn new(size: usize) -> Self {
        let shared = Mutex::new(State::new(size));
        let has_work = Condvar::new();
        Self {
            state: shared,
            has_work,
        }
    }
}
struct State {
    queue: Vec<i64>,
    shutting_down: bool,
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            queue: Vec::with_capacity(size),
            shutting_down: false,
        }
    }
}

fn do_task(sh: Arc<Shared>, task: Task) {
    loop {
        let maybe_item = {
            let mut state = sh.state.lock().expect("u'd be ok");

            while state.queue.is_empty() && !state.shutting_down {
                state = sh.has_work.wait(state).expect("u'd be ok");
            }

            if let Some(item) = state.queue.pop() {
                Some(item)
            } else if state.shutting_down {
                None
            } else {
                continue;
            }
        };

        match maybe_item {
            Some(item) => task(item),
            None => break,
        }
    }
}
