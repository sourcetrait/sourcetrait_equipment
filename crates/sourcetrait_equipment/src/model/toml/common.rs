use crate::*;



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlProvider {
    pub key: String,
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TomlSupportVersionReq {
    pub version: String,
}

