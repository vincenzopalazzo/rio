//! Printer trait that implement the logic
//! print a return given by an Extractor!

pub trait Printer<T> {
    /// Build a new printer!
    fn new(created: &str, since: &str, team: &str) -> Self;

    /// Take an input the content that can be
    /// the result of a API call and printify
    /// in a formatted string.
    fn printify(&self, content: &T) -> String;
}
