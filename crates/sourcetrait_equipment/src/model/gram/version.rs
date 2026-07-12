use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(pub semver::Version);

impl TryFrom<String> for Version {
    type Error = EquipmentError;
    fn try_from(ver: String) -> EquipmentResult<Self> {
        semver::Version::parse(&ver)
            .map(|v| Version(v))
            .map_err(|source| EquipmentError::SemVer { source, ver })
    }
}

impl FromFixed<(u64, u64, u64, Option<&str>, Option<&str>)> for Version {
    fn fixed(v: (u64, u64, u64, Option<&str>, Option<&str>)) -> Self {
        Self(semver::Version {
            major: v.0,
            minor: v.1,
            patch: v.2,
            pre: v.3.map_or_else(|| semver::Prerelease::EMPTY, |v| semver::Prerelease::new(v).expect("valid")),
            build: v.4.map_or_else(|| semver::BuildMetadata::EMPTY, |v| semver::BuildMetadata::new(v).expect("valid")),
        })
    }
}

impl FromFixed<(u64, u64, u64, u64)> for Version {
    fn fixed(v: (u64, u64, u64, u64)) -> Self {
        Self::fixed((v.0, v.1, v.2, Some(v.3.to_string().as_ref()), None))
    }
}

impl FromFixed<(u64, u64, u64)> for Version {
    fn fixed(v: (u64, u64, u64)) -> Self {
        Self::fixed((v.0, v.1, v.2, None, None))
    }
}

