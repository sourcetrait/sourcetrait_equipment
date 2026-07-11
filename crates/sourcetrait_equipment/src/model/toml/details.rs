use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DetailsToml {
    pub summary: String,
    pub keywords: Vec<String>,
}

