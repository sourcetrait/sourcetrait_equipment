use crate::*;

impl TryFrom<EquipmentToml> for Equipment {
    type Error = EquipmentError;
    fn try_from(v: EquipmentToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: Key::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            repositories: v.provider,
            author: Author::try_from(v.author)?,
            details: Details::try_from(v.details)?,
            license: License::try_from(v.license)?,
            exports: EquipmentExports::try_from(v.exports)?,
            support: EquipmentSupport::try_from(v.support)?,
        })
    }
}

impl TryFrom<EquipmentTomlSupport> for EquipmentSupport {
    type Error = EquipmentError;
    fn try_from(v: EquipmentTomlSupport) -> EquipmentResult<Self> {
        Ok(Self {
            nushell: VersionReq::try_from(v.nushell)?,
            equipment: VersionReq::try_from(v.equipment)?,
        })
    }
}

impl TryFrom<EquipmentExportsToml> for EquipmentExports {
    type Error = EquipmentError;
    fn try_from(v: EquipmentExportsToml) -> EquipmentResult<Self> {
        Ok(Self {
            rig: v.rig.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            bag: v.bag.into_iter()
                .map(|s| BagKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            rig: v.rig.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}

impl Equipment {
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(EquipmentToml::read(path)?)
    }
}
