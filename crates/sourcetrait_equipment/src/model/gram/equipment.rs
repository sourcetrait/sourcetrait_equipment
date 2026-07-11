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

