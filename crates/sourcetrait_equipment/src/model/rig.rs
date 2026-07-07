use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub provider: Provider,
    pub author: Author,
    pub description: Details,
    pub license: License,
    pub exports: RigExports,
    pub nushell: RigNushell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigExports {
    pub libraries: Vec<LibraryName>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigNushell {
    pub version: Version,
}
