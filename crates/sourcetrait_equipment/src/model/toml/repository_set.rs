use crate::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericRepositorySetToml<REPO> {
    pub key: String,
    pub read: Option<REPO>,
    pub write: Option<REPO>,
    pub store: Option<REPO>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum RepositorySetToml {
    Git(GenericRepositorySetToml<GitRepositoryToml>),
    Ipfs(GenericRepositorySetToml<IpfsRepositoryToml>),
}

impl<TOML, GRAM> TryFrom<GenericRepositorySetToml<TOML>> for GenericRepositorySet<GRAM>
where
    GRAM: RepositoryTrait + TryFrom<TOML, Error = EquipmentError>,
{
    type Error = EquipmentError;
    fn try_from(v: GenericRepositorySetToml<TOML>) -> EquipmentResult<Self> {
        Ok(Self {
            key: TriKey::try_from(v.key)?,
            read: v.read.map(|r| GRAM::try_from(r)).transpose()?,
            write: v.write.map(|w| GRAM::try_from(w)).transpose()?,
            store: v.store.map(|s| GRAM::try_from(s)).transpose()?,
        })
    }
}

impl TryFrom<RepositorySetToml> for RepositorySet {
    type Error = EquipmentError;
    fn try_from(v: RepositorySetToml) -> EquipmentResult<Self> {
        match v {
            RepositorySetToml::Git(repo) => 
                Ok(RepositorySet::Git(GenericRepositorySet::try_from(repo)?)),
            RepositorySetToml::Ipfs(repo) =>
                Ok(RepositorySet::Ipfs(GenericRepositorySet::try_from(repo)?)),
        }
    }
}