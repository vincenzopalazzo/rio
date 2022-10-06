#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
use log::{debug, info};
use rio_rt::runitime as rio;
use surf;

pub(crate) mod extractor;
mod github;

use extractor::Extractor;

async fn run(extractor: &impl extractor::Extractor<Output = String>) -> Result<(), surf::Error> {
    let content = extractor.search_new().await?;
    info!("{}", content);
    Ok(())
}

fn main() {
    env_logger::init();
    debug!("Here we go, we are all good");
    rio::block_on(async {
        let github = github::GithubExtractor::new();
        run(&github).await.unwrap()
    });
    rio::wait();
}
