//! Github Extractor implementation!
//!
//! Grab from Github issue all the new issue that are open from
//! a specific date, and collect then to generate a report with
//! a very short summary.
use crate::extractor::Extractor;
use log::{debug, info};
use surf;

pub(crate) struct GithubExtractor {}

impl GithubExtractor {
    /// Create a new instance of the Extractor
    pub fn new() -> Self {
        GithubExtractor {}
    }
}

impl Extractor for GithubExtractor {
    type Output = String;
    async fn search_new(&self) -> Result<String, surf::Error> {
        debug!("Running the https request");
        let mut res = surf::get("https://api.github.com/octocat").await?;
        let body = res.body_string().await?;
        info!("{}", body);
        Ok(body)
    }
}
