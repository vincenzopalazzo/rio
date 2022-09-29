//! Duration module provide a sequence of feature to work with the time duration.
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::SystemTime;

/// Future sleep function
pub struct Sleep {
    /// When the future is created
    now: SystemTime,
    /// Waiting time specified by the user.
    waiting_ms: u128,
}

impl Sleep {
    pub fn new(ms: u128) -> Self {
        Sleep {
            now: SystemTime::now(),
            waiting_ms: ms,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.now.elapsed().unwrap().as_millis() >= self.waiting_ms {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
