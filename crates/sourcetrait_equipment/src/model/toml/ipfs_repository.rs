use crate::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IpfsRepositoryToml {
    pub provider: String,
    pub ipns: String,
    pub cid: String,
}

impl TryFrom<IpfsRepositoryToml> for IpfsRepository {
    type Error = EquipmentError;
    fn try_from(v: IpfsRepositoryToml) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: Key::try_from(v.provider)?,
            ipns: IpnsName::try_from(v.ipns)?,
            cid: IpfsCid::try_from(v.cid)?,
        })
    }
}
