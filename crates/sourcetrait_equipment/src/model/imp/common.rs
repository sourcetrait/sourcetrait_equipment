use crate::*;

impl LibraryName {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .case(StrCase::Snake)
        .max_len(16);
}

impl TryFrom<String> for LibraryName {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.valid(v).map(|v| Self(v))
    }
}

impl From<&str> for LibraryName {
    fn from(v: &str) -> Self {
        Self::CHECK.valid(v.to_string()).map(|v| Self(v)).expect("valid")
    }
}

impl Key {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .case(StrCase::Snake)
        .max_len(16);
}

impl From<&str> for Key { fn from(v: &str) -> Self { Self(v.to_string()) } }

impl TryFrom<String> for BiKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let (a,b) = v.split_once('/')
            .map(|(a,b)| (a.to_string(), b.to_string()))
            .ok_or_else(|| EquipmentError::String { str: v, err: StrErr::Path })?;
        
        Ok(Self(Key::try_from(a)?, Key::try_from(b)?))
    }
}

impl From<(&str,&str)> for BiKey {
    fn from(v: (&str, &str)) -> Self {
        Self(
            Key::try_from(v.0.to_string()).expect("valid"),
            Key::try_from(v.1.to_string()).expect("valid")
        )
    }
}

impl From<&str> for Title { fn from(v: &str) -> Self { Self(v.to_string()) } }

impl From<&str> for Version {
    fn from(v: &str) -> Self {
        Self(semver::Version::parse(v).expect("semver"))
    }
}

impl TryFrom<String> for Key {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.valid(v).map(|v| Self(v))
    }
}

impl Title {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(16);
}

impl TryFrom<String> for Title {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.valid(v).map(|v| Self(v))
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

impl TryFrom<TomlAuthor> for Author {
    type Error = EquipmentError;
    fn try_from(v: TomlAuthor) -> EquipmentResult<Self> {
        Ok(Self {
            name: Key::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            email: Email::try_from(v.email)?,
        })
    }
}

impl TryFrom<TomlProvider> for Provider {
    type Error = EquipmentError;
    fn try_from(v: TomlProvider) -> EquipmentResult<Self> {
        todo!()
    }
}

impl Summary {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(120);
}

impl TryFrom<String> for Summary {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.valid(v).map(|v| Self(v))
    }
}

impl From<&str> for Summary {
    fn from(v: &str) -> Self {
        Self::CHECK.valid(v.to_string()).map(|v| Self(v)).expect("valid")
    }
}

impl Keyword {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .max_len(8)
        .case(StrCase::Snake);
}

impl TryFrom<String> for Keyword {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.valid(v).map(|v| Self(v))
    }
}

impl From<&str> for Keyword {
    fn from(v: &str) -> Self {
        Self::CHECK.valid(v.to_string()).map(|v| Self(v)).expect("valid")
    }
}

impl TryFrom<TomlDetails> for Details {
    type Error = EquipmentError;
    fn try_from(v: TomlDetails) -> EquipmentResult<Self> {
        Ok(Self {
            summary: Summary::try_from(v.summary)?,
            keywords: v.keywords.into_iter()
                .map(|k| Keyword::try_from(k))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}

impl TryFrom<TomlLicense> for License {
    type Error = EquipmentError;
    fn try_from(v: TomlLicense) -> EquipmentResult<Self> {
        Ok(Self {
            spdx: v.spdx.map(|v| Spdx::try_from(v)).transpose()?,
            file: v.file,
        })
    }
}

impl TryFrom<String> for Spdx {
    type Error = EquipmentError;
    fn try_from(spdx: String) -> EquipmentResult<Self> {
        spdx::Expression::parse(&spdx)
            .map(|v| Self(v))
            .map_err(|source| EquipmentError::Spdx { source, spdx })
    }
}

impl From<&str> for Spdx {
    fn from(spdx: &str) -> Self {
        spdx::Expression::parse(&spdx)
            .map(|v| Self(v))
            .expect("valid")
    }
}

impl Email {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .min_len(3)
        .max_len(255);

    pub fn check(s: &str) -> EquipmentResult<()> {
        Self::CHECK.check(s)?;
        
        let (local, domain) = {
            let mut parts = s.split('@');
            let local = parts.next()
                .ok_or_else(|| EquipmentError::Email { address: s.to_string() })?;
            let domain = parts.next()
                .ok_or_else(|| EquipmentError::Email { address: s.to_string() })?;
            if parts.count() != 0 {
                return Err(EquipmentError::Email { address: s.to_string() });
            }
            
            (local, domain)
        };
        
        if local.len() < 1 || domain.len() < 1 {
            return Err(EquipmentError::Email { address: s.to_string() });
        }

        Ok(())
    }

    pub(crate) fn valid(s: String) -> EquipmentResult<Self> {
        Self::check(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<String> for Email {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::valid(v)
    }
}

impl From<&str> for Email {
    fn from(v: &str) -> Self {
        Self::valid(v.to_string()).expect("valid email")
    }
}
