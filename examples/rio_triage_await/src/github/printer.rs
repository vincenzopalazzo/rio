use crate::github::model::NewIssue;
use crate::printer::Printer;
use std::vec::Vec;

pub struct MDPrinter;

impl Printer<Vec<NewIssue>> for MDPrinter {
    fn new() -> Self {
        MDPrinter
    }

    fn printify(&self, content: &Vec<NewIssue>) -> String {
        todo!()
    }
}
