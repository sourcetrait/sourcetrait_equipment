use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentExportsToml {
    pub rig: Vec<String>,
    pub gear_box: Vec<String>,
    pub gear_desk: Vec<String>,
    pub gear_lib: Vec<String>,
    pub bag: Vec<String>,
}

impl TryFrom<EquipmentExportsToml> for EquipmentExports {
    type Error = EquipmentError;
    fn try_from(v: EquipmentExportsToml) -> EquipmentResult<Self> {
        Ok(Self {
            rig: v.rig.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            gear_box: v.gear_box.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            gear_desk: v.gear_desk.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            gear_lib: v.gear_lib.into_iter()
                .map(|s| BiKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
            bag: v.bag.into_iter()
                .map(|s| BagKey::try_from(s))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}
