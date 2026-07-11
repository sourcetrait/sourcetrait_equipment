use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    pub key: BiKey,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub support: EquipmentSupport,
    pub repositories: Vec<RepositorySet>,
    pub exports: EquipmentExports,
    pub imports: EquipmentImports,
}

impl Equipment {
    pub fn read_toml<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(EquipmentToml::read(path)?)
    }
}
