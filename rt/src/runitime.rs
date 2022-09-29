//! Here we go, finally our runtime implementation,
//! this will be a toy but with a very big ambition to begin a
//! use runtime some day full od experimental idea that we want
//! to test and see what is the impact of it.
use std::collections::LinkedList;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;

use crate::spawner::Spawner;
use crate::task::Task;

pub(crate) type Queue = Arc<Mutex<LinkedList<Arc<Task>>>>;

/// Runtime definition
pub(crate) struct Runtime {
    task_queue: Queue,
    spawner: Spawner,
    /// Size of the runtime
    pub(crate) size: AtomicUsize,
}

/// Runtime implementation, this is where the magic happens!
impl Runtime {
    /// start the runtime by spowing the event look on a thread!
    fn start() {
        std::thread::spawn(|| loop {
            let task = match Runtime::get().pop_front() {
                Some(task) => task,
                None => continue,
            };

            if task.is_blocking() {
                while let Poll::Pending = task.poll() {}
            } else {
                if let Poll::Pending = task.poll() {
                    task.waker();
                }
            }
        });
    }

    pub fn get() -> &'static Runtime {
        INSTANCE.get_or_init(configure)
    }

    pub fn spawner() -> Spawner {
        Runtime::get().spawner.clone()
    }

    fn pop_front(&self) -> Option<Arc<Task>> {
        self.task_queue.lock().unwrap().pop_front()
    }
}
static INSTANCE: crate::lazy::Lazy<Runtime> = crate::lazy::Lazy::new();

/// Configure the runtime!
fn configure() -> Runtime {
    Runtime::start();
    let queue = Arc::new(Mutex::new(LinkedList::new()));
    Runtime {
        spawner: Spawner {
            queue: queue.clone(),
        },
        task_queue: queue,
        size: AtomicUsize::new(0),
    }
}

/// Spawn a non-blocking `Future` onto the `whorl` runtime
pub fn spawn(future: impl Future<Output = ()> + Send + Sync + 'static) {
    Runtime::spawner().spawn(future);
}
/// Block on a `Future` and stop others on the `whorl` runtime until this
/// one completes.
pub fn block_on(future: impl Future<Output = ()> + Send + Sync + 'static) {
    Runtime::spawner().spawn_blocking(future);
}
/// Block further execution of a program until all of the tasks on the
/// `whorl` runtime are completed.
pub fn wait() {
    let runtime = Runtime::get();
    while runtime.size.load(Ordering::Relaxed) > 0 {}
}
