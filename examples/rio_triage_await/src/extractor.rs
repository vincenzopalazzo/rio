//! Extractor interface to query a content from a specific source
use surf;

pub enum PrintFormat {
    Markdown,
}

/// Generic Extractor from a source that can be a
/// web API or a File ecc.
pub trait Extractor {
    type Output = String;
    async fn search_new(&self) -> Result<Self::Output, surf::Error>;

    /// Convert the result from the API and return a new
    /// value for the format specified.
    async fn printify(&self, out: &Self::Output, format: PrintFormat) -> String;
}
