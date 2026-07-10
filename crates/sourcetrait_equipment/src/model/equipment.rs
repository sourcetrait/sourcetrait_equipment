use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub support: EquipmentSupport,
    pub repositories: Vec<RepositorySet>,
    pub exports: EquipmentExports,
    pub imports: EquipmentImports,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentExports {
    pub rig: Vec<BiKey>,
    pub gear_box: Vec<BiKey>,
    pub gear_desk: Vec<BiKey>,
    pub gear_lib: Vec<BiKey>,
    pub bag: Vec<TriKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentSupport {
    pub nushell: VersionReq,
    pub equipment: VersionReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EquipmentImports {
    repositories: Vec<RepositorySet>,
}

