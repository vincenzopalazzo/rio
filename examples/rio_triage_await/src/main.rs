use log::{info, trace};
use rio_rt::runitime::{block_on, wait};
use surf;

async fn build_request() -> Result<(), surf::Error> {
    trace!("Running the https request");
    let mut res = surf::get("https://api.github.com/octocat").await?;
    info!("{}", res.body_string().await.unwrap());
    Ok(())
}

fn main() {
    trace!("Here we go, we are all good");
    block_on(async {
        trace!("Running the future!");
        build_request().await.unwrap();
    });
    wait();
}
