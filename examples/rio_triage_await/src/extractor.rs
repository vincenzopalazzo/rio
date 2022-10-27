//! Extractor interface to query a content from a specific source
use surf;

/// Generic Extractor from a source that can be a
/// web API or a File ecc.
pub trait Extractor {
    type Output = String;
    async fn search_new(&self) -> Result<Self::Output, surf::Error>;
}
