use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryToml {
    pub name: String,
    pub title: String,
    pub author: TomlAuthor,
    pub version: String,
    pub provider: TomlProvider,
    pub description: TomlDescription,
    pub license: TomlLicense,
    pub imports: LibraryTomlImports,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryTomlImports {
    pub libraries: Vec<LibraryTomlImport>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LibraryTomlImport {
    pub name: String,
    pub author: String,
    pub version: String,
    pub provider: String,
    pub path: Option<PathBuf>,
}
