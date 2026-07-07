use crate::*;

impl TryFrom<LibraryToml> for Library {
    type Error = EquipmentError;

    fn try_from(v: LibraryToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: Key::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            author: Author::try_from(v.author)?,
            details: Details::try_from(v.details)?,
            license: License::try_from(v.license)?, 
            imports: LibraryImports::try_from(v.imports)?,
        })
    }
}

impl TryFrom<LibraryTomlImports> for LibraryImports {
    type Error = EquipmentError;
    fn try_from(v: LibraryTomlImports) -> EquipmentResult<Self> {
        Ok(Self {
            libraries: v.library.into_iter()
                .map(|v| BiKey::try_from(v))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}

impl TryFrom<LibraryTomlImport> for LibraryImport {
    type Error = EquipmentError;
    fn try_from(v: LibraryTomlImport) -> EquipmentResult<Self> {
        Ok(Self {
            key: Key::try_from(v.key)?,
            author: Key::try_from(v.author)?,
            version: VersionReq::try_from(v.version)?,
            provider: Key::try_from(v.provider)?,
            path: v.path,
            
        })
    }
}

impl Library {
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(LibraryToml::read(path)?)
    }
}
