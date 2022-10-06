use log::{debug, info};
use rio_rt::runitime::{block_on, wait};
use surf;

async fn build_request() -> Result<(), surf::Error> {
    debug!("Running the https request");
    let mut res = surf::get("https://api.github.com/octocat").await?;
    info!("{}", res.body_string().await?);
    Ok(())
}

fn main() {
    env_logger::init();
    debug!("Here we go, we are all good");
    block_on(async {
        build_request().await.unwrap();
    });
    wait();
}
