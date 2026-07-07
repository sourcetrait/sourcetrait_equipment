use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlAuthor {
    pub key: String,
    pub title: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlDetails {
    pub summary: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlLicense {
    pub spdx: Option<String>,
    pub file: PathBuf,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlProvider {
    pub key: String,
    pub account: String,
    pub reference: String,
    pub mirror: TomlProviderMirror,
    pub contribute: TomlProviderContribute,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlProviderMirror {
    pub uri: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlProviderContribute {
    pub uri: String,
}
