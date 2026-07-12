use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: BiKey,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub imports: Vec<ImportEnum>,
}

impl Rig {
    pub fn read_toml<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(RigToml::read(path)?)
    }
}
