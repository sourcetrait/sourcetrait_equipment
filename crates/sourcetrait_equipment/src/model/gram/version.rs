use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(pub semver::Version);

impl From<&'static str> for Version {
    fn from(v: &'static str) -> Self {
        Self(semver::Version::parse(v).expect("semver"))
    }
}

impl TryFrom<String> for Version {
    type Error = EquipmentError;
    fn try_from(ver: String) -> EquipmentResult<Self> {
        semver::Version::parse(&ver)
            .map(|v| Version(v))
            .map_err(|source| EquipmentError::SemVer { source, ver })
    }
}
