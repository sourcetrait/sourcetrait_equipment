use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rig {
    pub key: Key,
    pub title: Title,
    pub version: Version,
    pub author: Author,
    pub details: Details,
    pub license: License,
    pub imports: RigImports,
}

