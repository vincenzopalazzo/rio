use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use std::io;

pub struct AdaptorFuture<F> {
    pub inner: F,
}

impl<F, T> Future for AdaptorFuture<F>
where
    F: Future<Output = T>,
{
    type Output = Result<T, io::Error>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        panic!()
    }
}

pub struct HyperFuture<B>
where
    B: ToString,
{
    inner: B,
}

impl<B> Future for HyperFuture<B>
where
    B: ToString,
{
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        panic!()
    }
}
