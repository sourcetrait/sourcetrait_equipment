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

impl LibraryToml {
    pub const LIBRARY_TOML: &'static str = "library.rig.toml";
    
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        let path = if path.as_ref().is_dir() {
            path.as_ref().join(Self::LIBRARY_TOML)
        } else {
            path.into()
        };
        
        let txt = fs::read_to_string(&path)
            .map_err(|source| EquipmentError::File { source, path: path.to_path_buf(), op: IoErr::Read })?;
        toml::from_str(&txt)
            .map_err(|source| EquipmentError::TomlFileRead { source, path: path.to_path_buf() })
    }
}