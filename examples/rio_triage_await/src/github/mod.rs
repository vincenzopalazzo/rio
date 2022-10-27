//! Github Extractor implementation!
//!
//! Grab from Github issue all the new issue that are open from
//! a specific date, and collect then to generate a report with
//! a very short summary.
use std::vec::Vec;

use crate::extractor::Extractor;
use crate::model::TriageConf;
use log::{debug, info};
use surf;

pub(crate) mod model;
pub(crate) mod printer;

use printer::MDPrinter;

pub(crate) struct GithubExtractor {
    owner: String,
    repo: String,
    since: String,
    labels: Vec<String>,
}

impl GithubExtractor {
    /// Create a new instance of the Extractor
    pub fn new(conf: &TriageConf) -> Self {
        GithubExtractor {
            owner: conf.git.owner.to_owned(),
            repo: conf.git.repo.to_owned(),
            since: conf.git.since.to_owned(),
            labels: conf.git.labels.to_owned(),
        }
    }

    fn apply_filers(&self, base_url: &mut String) {
        let mut labels = String::new();
        self.labels
            .iter()
            .for_each(|label| labels += format!("{label},").as_str());
        *base_url += format!("?q=created:>{}+labels={labels}", self.since).as_str();
    }

    pub fn printify(&self) -> String {
        todo!()
    }
}

impl Extractor for GithubExtractor {
    type Output = String;
    async fn search_new(&self) -> Result<String, surf::Error> {
        debug!("Running the https request");
        let api_url = "https://api.github.com/repos";
        let mut base_url = format!("{api_url}/{}/{}/issues", self.owner, self.repo);
        self.apply_filers(&mut base_url);
        let mut res = surf::get(base_url).await?;
        let body = res.body_string().await?;
        Ok(body)
    }
}

impl Clone for GithubExtractor {
    fn clone(&self) -> Self {
        GithubExtractor {
            owner: self.owner.to_string(),
            repo: self.repo.to_string(),
            since: self.since.to_owned(),
            labels: self.labels.clone(),
        }
    }
}
