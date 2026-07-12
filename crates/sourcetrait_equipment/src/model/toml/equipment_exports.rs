use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentExportsToml {
    pub rig: Vec<String>,
    pub gear_box: Vec<String>,
    pub gear_desk: Vec<String>,
    pub gear_lib: Vec<String>,
    pub bag: Vec<String>,
}

impl TryFrom<EquipmentExportsToml> for Vec<ExportEnum> {
    type Error = EquipmentError;
    fn try_from(v: EquipmentExportsToml) -> EquipmentResult<Self> {
        todo!()
    }
}
