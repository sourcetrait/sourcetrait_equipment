use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VersionSupportToml {
    pub version: String,
}
