use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigToml {
    pub key: String,
    pub title: String,
    pub author: AuthorToml,
    pub version: String,
    pub details: DetailsToml,
    pub license: LicenseToml,
    #[serde(alias = "import")]
    pub imports: ImportsToml,
}

impl TryFrom<RigToml> for Rig {
    type Error = EquipmentError;

    fn try_from(v: RigToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: BiKey::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            author: Author::try_from(v.author)?,
            details: Details::try_from(v.details)?,
            license: License::try_from(v.license)?, 
            imports: v.imports.try_into()?,
        })
    }
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

