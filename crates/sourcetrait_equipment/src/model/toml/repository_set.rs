use crate::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepositorySetToml<REPO> {
    pub key: String,
    pub read: Option<REPO>,
    pub write: Option<REPO>,
    pub store: Option<REPO>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum RepositorySetEnumToml {
    Git(RepositorySetToml<GitRepositoryToml>),
    Ipfs(RepositorySetToml<IpfsRepositoryToml>),
}

impl<TOML, GRAM> TryFrom<RepositorySetToml<TOML>> for RepositorySet<GRAM>
where
    GRAM: RepositoryTrait + TryFrom<TOML, Error = EquipmentError>,
{
    type Error = EquipmentError;
    fn try_from(v: RepositorySetToml<TOML>) -> EquipmentResult<Self> {
        Ok(Self {
            key: TriKey::try_from(v.key)?,
            read: v.read.map(|r| GRAM::try_from(r)).transpose()?,
            write: v.write.map(|w| GRAM::try_from(w)).transpose()?,
            store: v.store.map(|s| GRAM::try_from(s)).transpose()?,
        })
    }
}

impl TryFrom<RepositorySetEnumToml> for RepositorySetEnum {
    type Error = EquipmentError;
    fn try_from(v: RepositorySetEnumToml) -> EquipmentResult<Self> {
        match v {
            RepositorySetEnumToml::Git(repo) =>
                Ok(Self::Git(RepositorySet::try_from(repo)?)),
            RepositorySetEnumToml::Ipfs(repo) =>
                Ok(Self::Ipfs(RepositorySet::try_from(repo)?)),
        }
    }
}