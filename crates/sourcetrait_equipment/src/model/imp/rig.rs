use crate::*;

impl TryFrom<RigImportsToml> for RigImports {
    type Error = EquipmentError;
    fn try_from(v: RigImportsToml) -> EquipmentResult<Self> {
        Ok(Self {
            libraries: v.library.into_iter()
                .map(|v| BiKey::try_from(v))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}

impl TryFrom<RigImportToml> for RigImport {
    type Error = EquipmentError;
    fn try_from(v: RigImportToml) -> EquipmentResult<Self> {
        Ok(Self {
            key: BiKey::try_from(v.key)?,
            author: Key::try_from(v.author)?,
            version: VersionReq::try_from(v.version)?,
            provider: Key::try_from(v.provider)?,
            path: v.path,
        })
    }
}

