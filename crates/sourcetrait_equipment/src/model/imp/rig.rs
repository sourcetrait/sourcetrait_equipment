use crate::*;

impl TryFrom<RigToml> for Rig {
    type Error = EquipmentError;
    fn try_from(v: RigToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: Key::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            version: Version::try_from(v.version)?,
            provider: Provider::try_from(v.provider)?,
            author: Author::try_from(v.author)?,
            details: Details::try_from(v.details)?,
            license: License::try_from(v.license)?,
            exports: RigExports::try_from(v.exports)?,
            support: RigSupport::try_from(v.support)?,
        })
    }
}

impl TryFrom<RigTomlSupport> for RigSupport {
    type Error = EquipmentError;
    fn try_from(v: RigTomlSupport) -> EquipmentResult<Self> {
        Ok(Self {
            nushell: SupportVersionReq::try_from(v.nushell)?,
            equipment: SupportVersionReq::try_from(v.equipment)?,
        })
    }
}

impl TryFrom<RigTomlExports> for RigExports {
    type Error = EquipmentError;
    fn try_from(v: RigTomlExports) -> EquipmentResult<Self> {
        Ok(Self {
            libraries: v.libraries.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?
        })
    }
}

impl Rig {
    pub fn read<P: AsRef<Path> + Into<PathBuf>>(path: P) -> EquipmentResult<Self> {
        Self::try_from(RigToml::read(path)?)
    }
}
