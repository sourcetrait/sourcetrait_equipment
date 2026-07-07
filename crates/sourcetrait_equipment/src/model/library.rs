use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub name: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub provider: Provider,
    pub description: Details,
    pub license: License,
    pub imports: LibraryImports,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryImports {
    pub libraries: Vec<LibraryImport>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryImport {
    pub name: Key,
    pub author: Key,
    pub version: VersionReq,
    pub provider: Key,
    pub path: Option<PathBuf>,
}

impl TryFrom<LibraryToml> for Library {
    type Error = EquipmentError;

    fn try_from(v: LibraryToml) -> EquipmentResult<Self> {
        Ok(Self {
            name: Key::try_from(v.name)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            author: Author::try_from(v.author)?,
            provider: Provider::try_from(v.provider)?,
            description: Details::try_from(v.description)?,
            license: License::try_from(v.license)?, 
            imports: LibraryImports::try_from(v.imports)?,
        })
    }
}

impl TryFrom<LibraryTomlImports> for LibraryImports {
    type Error = EquipmentError;
    fn try_from(v: LibraryTomlImports) -> EquipmentResult<Self> {
        Ok(Self {
            libraries: v.libraries.into_iter()
                .map(|v| LibraryImport::try_from(v))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}

impl TryFrom<LibraryTomlImport> for LibraryImport {
    type Error = EquipmentError;
    fn try_from(v: LibraryTomlImport) -> EquipmentResult<Self> {
        Ok(Self {
            name: Key::try_from(v.name)?,
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
