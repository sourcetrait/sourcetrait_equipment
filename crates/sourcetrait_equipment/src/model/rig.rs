use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub imports: RigImports,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigImports {
    pub libraries: Vec<BiKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigImport {
    pub key: Key,
    pub author: Key,
    pub version: VersionReq,
    pub provider: Key,
    pub path: Option<PathBuf>,
}
