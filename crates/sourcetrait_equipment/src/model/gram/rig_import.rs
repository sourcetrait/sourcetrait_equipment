use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RigImport {
    pub key: BiKey,
    pub author: Key,
    pub version: VersionReq,
    pub provider: Key,
    pub path: Option<PathBuf>,
}
