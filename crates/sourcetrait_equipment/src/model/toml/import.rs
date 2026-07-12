use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigImportToml {
    pub key: String,
    pub repository: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GearBoxImportToml {
    pub key: String,
    pub repository: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GearDeskImportToml {
    pub key: String,
    pub repository: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GearLibImportToml {
    pub key: String,
    pub repository: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BagImportToml {
    pub key: String,
    pub repository: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ImportEnumToml {
    Rig(RigImportToml),
    GearBox(GearBoxImportToml),
    GearDesk(GearDeskImportToml),
    GearLib(GearLibImportToml),
    Bag(BagImportToml),
}