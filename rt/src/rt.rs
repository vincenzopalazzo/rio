//! A reference implementation of
//! the async main and tests proposed
//! discussed at https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async/topic/Weekly.20sync.202022-12-15
use std::future::Future;

/// Runtime trait definition is a
/// generic interface of the trait
/// that allow to have a generic
/// interface.
///
/// This allow the possibility to start
/// experimenting with this trait and
/// see if it is possible design library
/// executor agnostics.
pub trait Rt {
    fn new() -> &'static Self;

    /// Allow to run the future and block the execution till this
    /// future is ended.
    fn block_on(&self, future: impl Future<Output = ()> + Send + 'static);
}
