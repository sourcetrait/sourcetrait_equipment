use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentToml {
    pub key: String,
    pub title: String,
    pub version: String,
    pub author: AuthorToml,
    pub repositories: Vec<RepositorySetToml>,
    pub details: DetailsToml,
    pub license: LicenseToml,
    pub support: EquipmentSupportToml,
    pub exports: EquipmentExportsToml,
}

impl TryFrom<EquipmentToml> for Equipment {
    type Error = EquipmentError;
    fn try_from(v: EquipmentToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: BiKey::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            repositories: v.repositories.into_iter()
                .map(|v| RepositorySet::try_from(v))
                .collect()?,
            author: Author::try_from(v.author)?,
            details: Details::try_from(v.details)?,
            license: License::try_from(v.license)?,
            support: EquipmentSupport::try_from(v.support)?,
            imports: EquipmentImports::try_from(v.imports)?,
            exports: EquipmentExports::try_from(v.exports)?,
        })
    }
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