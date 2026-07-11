use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentSupportToml {
    pub nushell: VersionSupportToml,
    pub equipment: VersionSupportToml,
}

impl TryFrom<EquipmentSupportToml> for EquipmentSupport {
    type Error = EquipmentError;
    fn try_from(v: EquipmentSupportToml) -> EquipmentResult<Self> {
        Ok(Self {
            nushell: VersionReq::try_from(v.nushell)?,
            equipment: VersionReq::try_from(v.equipment)?,
        })
    }
}
