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
    Rig(RigExport),
    GearBox(GearBoxExport),
    GearDesk(GearDeskExport),
    GearLib(GearLibExport),
    Bag(BagExport),
}

impl From<RigExport> for ExportEnum {
    fn from(v: RigExport) -> Self { Self::Rig(v) }
}
impl From<GearBoxExport> for ExportEnum {
    fn from(v: GearBoxExport) -> Self { Self::GearBox(v) }
}
impl From<GearDeskExport> for ExportEnum {
    fn from(v: GearDeskExport) -> Self { Self::GearDesk(v) }
}
impl From<GearLibExport> for ExportEnum {
    fn from(v: GearLibExport) -> Self { Self::GearLib(v) }
}
impl From<BagExport> for ExportEnum {
    fn from(v: BagExport) -> Self { Self::Bag(v) }
}

impl FromFixed<(&'static str, &'static str)> for RigExport {
    fn fixed(v: (&'static str, &'static str)) -> Self {
        RigExport(BiKey::fixed((v.0, v.1)))
    }
}
