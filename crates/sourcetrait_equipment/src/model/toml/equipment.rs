use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentToml {
    pub key: String,
    pub title: String,
    pub version: String,
    pub author: AuthorToml,
    pub provider: TomlProvider,
    pub details: DetailsToml,
    pub license: LicenseToml,
    pub support: EquipmentSupportToml,
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
pub struct EquipmentSupportToml {
    pub nushell: TomlSupportVersionReq,
    pub equipment: TomlSupportVersionReq,
}

impl EquipmentToml {
    pub const EQUIPMENT_TOML: &'static str = "equipment.toml";
    
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        let path = match path.as_ref().is_dir() {
            true => path.as_ref().join(Self::EQUIPMENT_TOML),
            false => path.into(),
        };
        
        let txt = fs::read_to_string(&path)
            .map_err(|source| EquipmentError::File { source, path: path.to_path_buf(), op: IoErr::Read })?;
        toml::from_str(&txt)
            .map_err(|source| EquipmentError::TomlFileRead { source, path: path.to_path_buf() })
    }
}