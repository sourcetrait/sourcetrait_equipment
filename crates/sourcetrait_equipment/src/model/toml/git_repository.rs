use crate::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GitRepositoryToml {
    pub provider: String,
    pub uri: String,
    pub reference: String,
}

impl TryFrom<GitRepositoryToml> for GitRepository {
    type Error = EquipmentError;
    fn try_from(v: GitRepositoryToml) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: Key::try_from(v.provider)?,
            uri: GitUri::try_from(v.uri)?,
            reference: GitReference::try_from(v.uri)?,
        })
    }
}

