#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
#![feature(inherent_associated_types)]
use extractor::PrintFormat;
use github::model::NewIssue;
use hackmd::api::HackmdAPI;
use hackmd::model::NewNote;
use log::debug;
use rio_rt::runitime::Runtime;
use rio_rt::Rt;
use surf;

pub(crate) mod extractor;
mod github;
pub mod hackmd;
mod model;
pub(crate) mod printer;

async fn run(
    extractor: &impl extractor::Extractor<Output = Vec<NewIssue>>,
    hackmd_api: &HackmdAPI,
) -> Result<(), surf::Error> {
    let content = extractor.search_new().await?;
    let result = extractor.printify(&content, PrintFormat::Markdown).await;

    let opts = NewNote::new(&result);
    hackmd_api.new_note(&opts).await?;
    Ok(())
}

fn main() {
    env_logger::init();
    let rio = Runtime::new();

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
    let hackmd_api = hackmd::api::HackmdAPI::new("", false);

    rio.block_on(async move {
        run(&github, &hackmd_api).await.unwrap();
    });
}
