use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentToml {
    pub key: String,
    pub title: String,
    pub version: String,
    pub author: TomlAuthor,
    pub provider: TomlProvider,
    pub details: TomlDetails,
    pub license: TomlLicense,
    pub support: EquipmentTomlSupport,
    pub exports: EquipmentExportsToml,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentExportsToml {
    pub rig: Vec<String>,
    pub gear_box: Vec<String>,
    pub gear_desk: Vec<String>,
    pub gear_lib: Vec<String>,
    pub bag: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentTomlSupport {
    pub nushell: TomlSupportVersionReq,
    pub equipment: TomlSupportVersionReq,
}

impl EquipmentToml {
    pub const RIG_TOML: &'static str = "rig.toml";
    
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        let path = if path.as_ref().is_dir() {
            path.as_ref().join(Self::RIG_TOML)
        } else {
            path.into()
        };
        
        let txt = fs::read_to_string(&path)
            .map_err(|source| EquipmentError::File { source, path: path.to_path_buf(), op: IoErr::Read })?;
        let rig = toml::from_str(&txt)
            .map_err(|source| EquipmentError::TomlFileRead { source, path: path.to_path_buf() })?;
        
        Ok(rig)
    }
}