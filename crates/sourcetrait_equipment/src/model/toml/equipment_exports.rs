use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentExportsToml {
    pub rig: Vec<String>,
    pub gear_box: Vec<String>,
    pub gear_desk: Vec<String>,
    pub gear_lib: Vec<String>,
    pub bag: Vec<String>,
}

