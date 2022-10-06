///! Extractor interface to query a content from a specific source
use surf;

pub trait Extractor {
    type Output = String;
    async fn search_new(&self) -> Result<Self::Output, surf::Error>;
}
