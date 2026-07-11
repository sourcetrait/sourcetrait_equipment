use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentSupport {
    pub nushell: VersionReq,
    pub equipment: VersionReq,
}

