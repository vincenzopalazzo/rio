//! Github Extractor implementation!
//!
//! Grab from Github issue all the new issue that are open from
//! a specific date, and collect then to generate a report with
//! a very short summary.
use std::time::SystemTime;
use std::vec::Vec;

use crate::extractor::{Extractor, PrintFormat};
use crate::model::TriageConf;
use crate::printer::Printer;
use chrono::offset::Utc;
use chrono::DateTime;
use log::{debug, info, trace};
use surf;

pub(crate) mod model;
pub(crate) mod printer;

use printer::MDPrinter;

use self::model::NewIssue;

pub(crate) struct GithubExtractor {
    team: String,
    owner: String,
    repo: String,
    since: String,
    labels: Vec<String>,
}

impl GithubExtractor {
    /// Create a new instance of the Extractor
    pub fn new(conf: &TriageConf) -> Self {
        GithubExtractor {
            team: conf.team.to_owned(),
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
        debug!("Filter labels: {labels} for since {}", self.since);
        labels = labels.strip_suffix(",").unwrap_or(&labels).to_owned();
        *base_url += format!("?labels={labels}&since={}", self.since).as_str();
        debug!("URL with filtering {base_url}");
    }
}

impl Extractor for GithubExtractor {
    type Output = Vec<NewIssue>;
    async fn search_new(&self) -> Result<Self::Output, surf::Error> {
        info!("Fetch new issue from Github");
        let api_url = "https://api.github.com/repos";
        let mut base_url = format!("{api_url}/{}/{}/issues", self.owner, self.repo);
        self.apply_filers(&mut base_url);
        let mut res = surf::get(base_url).await?;
        let body = res.body_string().await?;
        trace!("API response: {body}");
        let our: Vec<NewIssue> = serde_json::from_str(&body).unwrap();
        Ok(our)
    }

    async fn printify(&self, out: &Self::Output, format: PrintFormat) -> String {
        match format {
            PrintFormat::Markdown => {
                let now = SystemTime::now();
                let datetime: DateTime<Utc> = now.into();
                let formatter = MDPrinter::new(
                    datetime.format("%d/%m/%Y").to_string().as_str(),
                    &self.since,
                    &self.team,
                );
                formatter.printify(out)
            }
        }
    }
}

impl Clone for GithubExtractor {
    fn clone(&self) -> Self {
        GithubExtractor {
            team: self.team.to_string(),
            owner: self.owner.to_string(),
            repo: self.repo.to_string(),
            since: self.since.to_owned(),
            labels: self.labels.clone(),
        }
    }
}
