use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LicenseToml {
    pub spdx: Option<String>,
    pub file: PathBuf,
}
