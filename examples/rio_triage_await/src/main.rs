#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
use extractor::PrintFormat;
use github::model::NewIssue;
use log::debug;
use rio_rt::runitime as rio;
use surf;

pub(crate) mod extractor;
mod github;
mod model;
pub(crate) mod printer;

async fn run(
    extractor: &impl extractor::Extractor<Output = Vec<NewIssue>>,
) -> Result<(), surf::Error> {
    let content = extractor.search_new().await?;
    let result = extractor.printify(&content, PrintFormat::Markdown).await;
    debug!("\n{result}");
    Ok(())
}

fn main() {
    env_logger::init();
    debug!("Here we go, we are all good");

    // FIXME: load conf from json

    let conf = model::TriageConf {
        team: "async-wg".to_owned(),
        git: model::GitConf {
            owner: "rust-lang".to_owned(),
            repo: "rust".to_owned(),
            since: "2022-11-7T19:27:47Z".to_owned(),
            labels: vec!["A-async-await".to_owned()],
        },
    };

    let github = github::GithubExtractor::new(&conf);
    rio::block_on(async move {
        run(&github).await.unwrap();
    });

    rio::wait();
}
