//! Task implementation
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

use crate::runitime::Runtime;

type PinFuture = Mutex<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>;

/// The `Task` is the basic unit for the executor. It represents a `Future`
/// that may or may not be completed. We spawn `Task`s to be run and poll
/// them until completion in a non-blocking manner unless specifically asked
/// for.
pub(crate) struct Task {
    /// This is the actual `Future` we will poll inside of a `Task`. We `Box`
    /// and `Pin` the `Future` when we create a task so that we don't need
    /// to worry about pinning or more complicated things in the runtime. We
    /// also need to make sure this is `Send + Sync` so we can use it across threads
    /// and so we lock the `Pin<Box<dyn Future>>` inside a `Mutex`.
    future: PinFuture,
    /// We need a way to check if the runtime should block on this task and
    /// so we use a boolean here to check that!
    block: bool,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        if self.is_blocking() {
            Runtime::spawner().inner_spawn_blocking(self);
        } else {
            Runtime::spawner().inner_spawn(self);
        }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        Runtime::get().size.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Task {
    pub(crate) fn new(
        block: bool,
        future: impl Future<Output = ()> + Send + Sync + 'static,
    ) -> Arc<Self> {
        Runtime::get().size.fetch_add(1, Ordering::Relaxed);
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            block,
        })
    }

    /// Pool the following task!
    pub fn poll(self: &Arc<Self>) -> Poll<()> {
        let waker = self.waker();
        let mut ctx = Context::from_waker(&waker);
        self.future.lock().unwrap().as_mut().poll(&mut ctx)
    }

    // FIXIME: what is this method?
    pub fn waker(self: &Arc<Self>) -> Waker {
        self.clone().into()
    }

    /// The Task is blocking.
    pub fn is_blocking(&self) -> bool {
        self.block
    }
}
