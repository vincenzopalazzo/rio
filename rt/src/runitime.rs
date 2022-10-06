//! Here we go, finally our runtime implementation,
//! this will be a toy but with a very big ambition to begin a
//! use runtime some day full od experimental idea that we want
//! to test and see what is the impact of it.
use std::collections::LinkedList;
use std::future::Future;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::task::Poll;
use std::task::Wake;

use log::info;

use crate::task::Task;

pub(crate) type Queue = Arc<Mutex<LinkedList<Arc<Task>>>>;

/// Runtime definition
pub(crate) struct Runtime {
    pub(crate) task_queue: Queue,
    /// Size of the runtime
    pub(crate) size: AtomicUsize,
}

/// Runtime implementation, this is where the magic happens!
impl Runtime {
    /// start the runtime by spowing the event look on a thread!
    fn start() {
        std::thread::spawn(|| {
            while !Runtime::is_empty() {
                info!("Size Task: {}", Runtime::get().size.load(Ordering::Relaxed));
                let task = match Runtime::get().pop_front() {
                    Some(task) => task,
                    None => continue,
                };

                if let Poll::Pending = task.poll() {
                    task.wake();
                }
            }
        });
    }

    pub(crate) fn is_empty() -> bool {
        Runtime::get().task_queue.lock().unwrap().is_empty()
    }

    pub fn get() -> &'static Runtime {
        INSTANCE.deref()
    }

    fn pop_front(&self) -> Option<Arc<Task>> {
        self.task_queue.lock().unwrap().pop_front()
    }

    /// This is the function that gets called by the `spawn` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the back of the queue.
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.inner_spawn(Task::new(false, future));
    }
    /// This is the function that gets called by the `spawn_blocking` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the front of the queue
    /// where the runtime will check if it should block and then block until
    /// this future completes.
    pub fn spawn_blocking(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.inner_spawn_blocking(Task::new(true, future));
    }

    /// This function just takes a `Task` and pushes it onto the queue. We use this
    /// both for spawning new `Task`s and to push old ones that get woken up
    /// back onto the queue.
    pub(crate) fn inner_spawn(&self, task: Arc<Task>) {
        self.task_queue.lock().unwrap().push_back(task);
    }

    /// This function takes a `Task` and pushes it to the front of the queue
    /// if it is meant to block. We use this both for spawning new blocking
    /// `Task`s and to push old ones that get woken up back onto the queue.
    pub(crate) fn inner_spawn_blocking(&self, task: Arc<Task>) {
        self.task_queue.lock().unwrap().push_front(task);
    }
}

pub(crate) static INSTANCE: LazyLock<Runtime> = LazyLock::new(|| configure());

/// Configure the runtime!
fn configure() -> Runtime {
    Runtime::start();
    let queue = Arc::new(Mutex::new(LinkedList::new()));
    Runtime {
        task_queue: queue.to_owned(),
        size: AtomicUsize::new(0),
    }
}

/// Spawn a non-blocking `Future` onto the `whorl` runtime
pub fn spawn(future: impl Future<Output = ()> + Send + 'static) {
    Runtime::get().spawn(future);
}
/// Block on a `Future` and stop others on the `whorl` runtime until this
/// one completes.
pub fn block_on(future: impl Future<Output = ()> + Send + 'static) {
    Runtime::get().spawn_blocking(future);
}
/// Block further execution of a program until all of the tasks on the
/// `whorl` runtime are completed.
pub fn wait() {
    let runtime = Runtime::get();
    while runtime.size.load(Ordering::Relaxed) > 0 {}
}
