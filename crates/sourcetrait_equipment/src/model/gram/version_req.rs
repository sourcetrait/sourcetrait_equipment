use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq(pub semver::VersionReq);

impl TryFrom<&str> for VersionReq {
    type Error = EquipmentError;
    fn try_from(v: &str) -> EquipmentResult<Self> {
        semver::VersionReq::parse(&v)
            .map(|v| VersionReq(v))
            .map_err(|source| EquipmentError::SemVer { source, ver: v.to_string() })
    }
}

impl TryFrom<String> for VersionReq {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::try_from(v.as_ref())
    }
}

impl FromFixed<&'static str> for VersionReq {
    fn fixed(v: &'static str) -> Self {
        Self::try_from(v).expect("valid")
    }
}