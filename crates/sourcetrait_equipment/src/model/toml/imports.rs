use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImportsToml {
    #[serde(alias = "rigs")]
    pub rig: Vec<RigImportToml>,
    #[serde(alias = "bags")]
    pub bag: Vec<BagImportToml>,
    pub gear: Option<GearImportsToml>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GearImportsToml {
    #[serde(rename = "box")]
    pub box_gear: Vec<GearBoxImportToml>,
    #[serde(rename = "desk")]
    pub desk_gear: Vec<GearDeskImportToml>,
    #[serde(rename = "lib")]
    pub lib_gear: Vec<GearLibImportToml>,
}
    
impl TryFrom<ImportsToml> for Vec<ImportEnum> {
    type Error = EquipmentError;
    fn try_from(value: ImportsToml) -> EquipmentResult<Self> {
        todo!()
    }
}