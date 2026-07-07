use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub imports: LibraryImports,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryImports {
    pub libraries: Vec<BiKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryImport {
    pub key: Key,
    pub author: Key,
    pub version: VersionReq,
    pub provider: Key,
    pub path: Option<PathBuf>,
}
