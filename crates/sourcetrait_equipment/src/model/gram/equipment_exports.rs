use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentExports {
    pub rig: Vec<BiKey>,
    pub gear_box: Vec<BiKey>,
    pub gear_desk: Vec<BiKey>,
    pub gear_lib: Vec<BiKey>,
    pub bag: Vec<BagKey>,
}

