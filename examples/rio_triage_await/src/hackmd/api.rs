//! API interface for hackmd.
use crate::hackmd::model::NewNote;
use surf;

pub struct HackmdAPI {
    token: String,
    team: bool,
}

impl HackmdAPI {
    type Err = surf::Error;

    pub fn new(token: &str, team: bool) -> Self {
        HackmdAPI {
            token: token.to_owned(),
            team,
        }
    }

    fn build_base_url(&self) -> String {
        let base = "https://api.hackmd.io/v1/";
        if self.team {
            return format!("{base}/teams");
        }
        return base.to_owned();
    }

    pub async fn new_note(&self, opts: &NewNote) -> Result<(), Self::Err> {
        let base = self.build_base_url();
        let url = format!("{}/notes", base);
        let body_str = serde_json::to_string(opts).unwrap();
        surf::post(url).body(body_str).await?;
        Ok(())
    }
}
