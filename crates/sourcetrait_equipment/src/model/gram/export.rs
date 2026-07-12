use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigExport(pub BiKey);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearBoxExport(pub BiKey);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearDeskExport(pub BiKey);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearLibExport(pub BiKey);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BagExport(pub BagKey);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportEnum {
    RigExport(RigExport),
    GearBoxExport(GearBoxExport),
    GearDeskExport(GearDeskExport),
    GearLibExport(GearLibExport),
    BagExport(BagExport),
}
