use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub provider: Provider,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub exports: RigExports,
    pub support: RigSupport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigExports {
    pub libraries: Vec<BiKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigSupport {
    pub nushell: SupportVersionReq,
    pub equipment: SupportVersionReq,
}
