use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryToml {
    pub key: String,
    pub title: String,
    pub author: TomlAuthor,
    pub version: String,
    pub details: TomlDetails,
    pub license: TomlLicense,
    pub imports: LibraryTomlImports,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryTomlImports {
    pub library: Vec<String>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryTomlImport {
    pub key: String,
    pub author: String,
    pub version: String,
    pub provider: String,
    pub path: Option<PathBuf>,
}
