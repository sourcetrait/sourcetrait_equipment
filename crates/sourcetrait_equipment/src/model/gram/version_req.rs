use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq(pub semver::VersionReq);

impl TryFrom<String> for VersionReq {
    type Error = EquipmentError;
    fn try_from(ver: String) -> EquipmentResult<Self> {
        semver::VersionReq::parse(&ver)
            .map(|v| VersionReq(v))
            .map_err(|source| EquipmentError::SemVer { source, ver })
    }
}

impl From<&str> for VersionReq {
    fn from(ver: &str) -> Self {
        Self(semver::VersionReq::parse(&ver).expect("valid"))
    }
}
