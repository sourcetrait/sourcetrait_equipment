use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigToml {
    pub key: String,
    pub title: String,
    pub author: AuthorToml,
    pub version: String,
    pub details: DetailsToml,
    pub license: LicenseToml,
    pub imports: RigImportsToml,
}
impl RigToml {
    pub const RIG_TOML: &'static str = "rig.toml";
    
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        let path = if path.as_ref().is_dir() {
            path.as_ref().join(Self::RIG_TOML)
        } else {
            path.into()
        };
        
        let txt = fs::read_to_string(&path)
            .map_err(|source| EquipmentError::File { source, path: path.to_path_buf(), op: IoErr::Read })?;
        toml::from_str(&txt)
            .map_err(|source| EquipmentError::TomlFileRead { source, path: path.to_path_buf() })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigImportsToml {
    #[serde(alias = "libraries")]
    pub library: Vec<String>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigImportToml {
    pub key: String,
    pub author: String,
    pub version: String,
    pub provider: String,
    pub path: Option<PathBuf>,
}
