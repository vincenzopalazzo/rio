use crate::future::HyperFuture;
use crate::AdaptorFuture;
use std::future::Future;

use std::fmt::Display;
use std::io;

pub trait Service<R> {
    type Response;

    type Future: Future<Output = Self::Response>;

    fn call(&self, req: R) -> Self::Future;
}

pub struct HyperService<S> {
    request_service: S,
}

impl<S> HyperService<S> {
    pub fn new(request_service: S) -> Self {
        HyperService { request_service }
    }
}

impl<S, R> Service<R> for HyperService<S>
where
    S: ToString,
{
    type Response = Result<(), io::Error>;

    type Future = HyperFuture<String>;

    fn call(&self, _: R) -> Self::Future {
        panic!()
    }
}
