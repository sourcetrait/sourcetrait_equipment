use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Details {
    pub summary: Summary,
    pub keywords: Vec<Keyword>,
}
