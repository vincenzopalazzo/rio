//! API model for the hackmd API.
use serde::Serialize;
use surf::http::content;

#[derive(Serialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
    #[serde(rename = "readPermission")]
    pub read_permission: String,
    #[serde(rename = "writePermission")]
    pub write_permission: String,
    #[serde(rename = "commentPermission")]
    pub comment_permission: String,
}

impl NewNote {
    pub fn new(content: &str) -> Self {
        NewNote {
            title: String::new(),
            content: content.to_owned(),
            read_permission: "owner".to_owned(),
            write_permission: "owner".to_owned(),
            comment_permission: "everyone".to_owned(),
        }
    }
}
