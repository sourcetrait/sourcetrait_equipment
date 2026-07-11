use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigImportToml {
    pub key: String,
    pub author: String,
    pub version: String,
    pub provider: String,
    pub path: Option<PathBuf>,
}
