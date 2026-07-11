use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub imports: RigImports,
}

impl Rig {
    pub fn read_toml<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(LibraryToml::read(path)?)
    }
}
