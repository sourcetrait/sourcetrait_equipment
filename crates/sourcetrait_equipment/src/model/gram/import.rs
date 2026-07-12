use crate::*;

pub trait ImportTrait<KEY: Sized>: Sized {
    //fn kind(&self) -> ImportKind;
    fn key(&self) -> &KEY;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigImport {
    pub key: BiKey,
    pub repository: TriKey,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GearBoxImport {
    pub key: BiKey,
    pub repository: TriKey,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GearDeskImport {
    pub key: BiKey,
    pub repository: TriKey,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GearLibImport {
    pub key: BiKey,
    pub repository: Key,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BagImport {
    pub key: BagKey,
    pub repository: Key,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportEnum {
    Rig(RigImport),
    GearBox(GearBoxImport),
    GearDesk(GearDeskImport),
    GearLib(GearLibImport),
    Bag(BagImport),
}

impl From<RigImport> for ImportEnum {
    fn from(v: RigImport) -> Self { Self::Rig(v) }
}
impl From<GearBoxImport> for ImportEnum {
    fn from(v: GearBoxImport) -> Self { Self::GearBox(v) }
}
impl From<GearDeskImport> for ImportEnum {
    fn from(v: GearDeskImport) -> Self { Self::GearDesk(v) }
}
impl From<GearLibImport> for ImportEnum {
    fn from(v: GearLibImport) -> Self { Self::GearLib(v) }
}
impl From<BagImport> for ImportEnum {
    fn from(v: BagImport) -> Self { Self::Bag(v) }
}
