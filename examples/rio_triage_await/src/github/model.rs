use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewIssue {
    pub html_url: String,
    pub number: u64,
    pub title: String,
    pub body: String,
    pub labels: IssueLabel,
    pub assignee: IssueAssigned,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueLabel {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssigned {
    login: String,
}
