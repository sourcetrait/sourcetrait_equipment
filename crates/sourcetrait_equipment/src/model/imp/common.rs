use crate::*;

impl TryFrom<TomlProvider> for Provider {
    type Error = EquipmentError;
    fn try_from(v: TomlProvider) -> EquipmentResult<Self> {
        Ok(Self {
            key: BiKey::try_from(v.key)?,
            reference: GitReference::try_from(v.reference)?,
            mirror: ProviderMirror::try_from(v.mirror)?,
            contribute: ProviderContribute::try_from(v.contribute)?
        })
    }
}

impl TryFrom<TomlProviderMirror> for ProviderMirror {
    type Error = EquipmentError;
    fn try_from(v: TomlProviderMirror) -> EquipmentResult<Self> {
        Ok(Self {
            uri: GitUri::try_from(v.uri)?,
        })
    }
}

impl TryFrom<TomlProviderContribute> for ProviderContribute {
    type Error = EquipmentError;
    fn try_from(v: TomlProviderContribute) -> EquipmentResult<Self> {
        Ok(Self {
            uri: GitUri::try_from(v.uri)?,
        })
    }
}


impl TryFrom<TomlSupportVersionReq> for VersionReq {
    type Error = EquipmentError;
    fn try_from(v: TomlSupportVersionReq) -> EquipmentResult<Self> {
        Ok(Self::try_from(v.version)?)
    }
}
