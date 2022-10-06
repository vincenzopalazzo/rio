//! Spawner crate implementation
use std::collections::LinkedList;
use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::runitime::Queue;
use crate::task::Task;

#[derive(Clone)]
pub(crate) struct Spawner {
    pub(crate) queue: Queue,
}
impl Spawner {
    pub(crate) fn new() -> Self {
        Spawner {
            queue: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    /// This is the function that gets called by the `spawn` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the back of the queue.
    pub fn spawn(self, future: impl Future<Output = ()> + Send + 'static) {
        self.inner_spawn(Task::new(false, future));
    }
    /// This is the function that gets called by the `spawn_blocking` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the front of the queue
    /// where the runtime will check if it should block and then block until
    /// this future completes.
    pub fn spawn_blocking(self, future: impl Future<Output = ()> + Send + 'static) {
        self.inner_spawn_blocking(Task::new(true, future));
    }
    /// This function just takes a `Task` and pushes it onto the queue. We use this
    /// both for spawning new `Task`s and to push old ones that get woken up
    /// back onto the queue.
    pub fn inner_spawn(self, task: Arc<Task>) {
        self.queue.lock().unwrap().push_back(task);
    }
    /// This function takes a `Task` and pushes it to the front of the queue
    /// if it is meant to block. We use this both for spawning new blocking
    /// `Task`s and to push old ones that get woken up back onto the queue.
    pub fn inner_spawn_blocking(self, task: Arc<Task>) {
        self.queue.lock().unwrap().push_front(task);
    }
}
