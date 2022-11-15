use chrono::{DateTime, Utc};

use crate::github::model::NewIssue;
use crate::printer::Printer;
use std::str::FromStr;
use std::vec::Vec;

pub struct MDPrinter {
    created: String,
    team: String,
    since: String,
    // FIXME: add zulip stream
}

impl Printer<Vec<NewIssue>> for MDPrinter {
    fn new(created: &str, since: &str, team: &str) -> Self {
        MDPrinter {
            created: created.to_owned(),
            team: team.to_owned(),
            since: since.to_owned(),
        }
    }

    fn printify(&self, issues: &Vec<NewIssue>) -> String {
        let mut content = String::new();
        content += "# Triage Meeting\n\n";
        content += format!("- Owner: {}\n", self.team).as_str();
        content += format!("- Date: {}\n", self.created).as_str();

        content += "\n\n";
        content += "## New Issues\n";
        for issue in issues {
            let created: DateTime<Utc> = DateTime::from_str(&issue.created_at).unwrap();
            let since: DateTime<Utc> = DateTime::from_str(&self.since).unwrap();
            if created.ge(&since) {
                content += format!(
                    "- [{}]({}) in date {}\n",
                    issue.title, issue.html_url, issue.created_at
                )
                .as_str();
            }
        }
        content
    }
}
