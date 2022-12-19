use rio_rt::runitime as rio;

mod future;
mod services;

use crate::future::AdaptorFuture;
use crate::services::{HyperService, Service};

fn main() {
    let service = HyperService::new("".to_string());
    rio::block_on(async move {
        service.call("".to_string());
    })
}
