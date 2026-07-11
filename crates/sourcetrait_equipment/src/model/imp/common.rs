use crate::*;


impl TryFrom<(&str,&str,&str)> for TriKey {
    type Error = EquipmentError;
    fn try_from(v: (&str, &str, &str)) -> EquipmentResult<Self> {
        Ok(Self(
            Key::try_from(v.0.to_string())?,
            Key::try_from(v.1.to_string())?,
            Key::try_from(v.2.to_string())?,
        ))
    }
}

impl TryFrom<(&str,&str,&str,&str)> for QuadKey {
    type Error = EquipmentError;
    fn try_from(v: (&str, &str, &str, &str)) -> EquipmentResult<Self> {
        Ok(Self(
            Key::try_from(v.0.to_string())?,
            Key::try_from(v.1.to_string())?,
            Key::try_from(v.2.to_string())?,
            Key::try_from(v.3.to_string())?,
        ))
    }
}

impl TryFrom<String> for TriKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let [a,b,c] = v.split('/').collect::<Vec<_>>()[..] else {
            return Err(EquipmentError::String { str: v, err: StrErr::Path });
        };
        
        Ok(Self::try_from((a,b,c))?)
    }
}

impl TryFrom<String> for QuadKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let [a,b,c,d] = v.split('/').collect::<Vec<_>>()[..] else {
            return Err(EquipmentError::String { str: v, err: StrErr::Path });
        };
        
        Ok(Self::try_from((a,b,c,d))?)
    }
}

impl TryFrom<String> for BagKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        match v.split('/').collect::<Vec<_>>()[..] {
            [a,b,c,d] => Ok(Self::Quad(QuadKey::try_from((a,b,c,d))?)),
            [a,b,c] => Ok(Self::Tri(TriKey::try_from((a,b,c))?)),
            _ => Err(EquipmentError::String { str: v, err: StrErr::Path }),
        }
    }
}

impl From<&str> for Title { fn from(v: &str) -> Self { Self(v.to_string()) } }

impl From<&'static str> for Version {
    fn from(v: &'static str) -> Self {
        Self(semver::Version::parse(v).expect("semver"))
    }
}

impl Title {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(32);
}

impl TryFrom<String> for Title {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
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
        Ok(Self {
            key: BiKey::try_from(v.key)?,
            reference: GitReference::try_from(v.reference)?,
            mirror: ProviderMirror::try_from(v.mirror)?,
            contribute: ProviderContribute::try_from(v.contribute)?
        })
    }
}

impl TryFrom<TomlProviderMirror> for ProviderMirror {
    type Error = EquipmentError;
    fn try_from(v: TomlProviderMirror) -> EquipmentResult<Self> {
        Ok(Self {
            uri: GitUri::try_from(v.uri)?,
        })
    }
}

impl TryFrom<TomlProviderContribute> for ProviderContribute {
    type Error = EquipmentError;
    fn try_from(v: TomlProviderContribute) -> EquipmentResult<Self> {
        Ok(Self {
            uri: GitUri::try_from(v.uri)?,
        })
    }
}

impl Summary {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(120);
}

impl TryFrom<String> for Summary {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}

impl From<&str> for Summary {
    fn from(v: &str) -> Self {
        Self::CHECK.validate(v.to_string()).map(|v| Self(v)).expect("valid")
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
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}

impl From<&str> for Keyword {
    fn from(v: &str) -> Self {
        Self::CHECK.validate(v.to_string()).map(|v| Self(v)).expect("valid")
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
        Self::try_from(v.to_string()).expect("valid email")
    }
}

impl GitUri {
    pub(crate) const HTTPS: &'static str = "https";
    pub(crate) const SSH: &'static str = "ssh";
}

impl TryFrom<String> for GitUri {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let url = url::Url::parse(&v)
            .map_err(|e| EquipmentError::url(e, v))?;

        match url.scheme() {
            GitUri::HTTPS => Ok(Self::Https(url)),
            GitUri::SSH => Ok(Self::Ssh(url)),
            p => Err(EquipmentError::ProviderProtocol { protocol: Some(p.to_string()) }),
        }
    }
}

impl From<&str> for GitUri {
    fn from(v: &str) -> Self {
        Self::try_from(v.to_string()).expect("valid")
    }
}

impl TryFrom<String> for GitReference {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Ok(Self(v))
    }
}

impl From<&str> for GitReference {
    fn from(v: &str) -> Self { 
        Self::try_from(v.to_string()).expect("valid")
    }
}

impl TryFrom<TomlSupportVersionReq> for VersionReq {
    type Error = EquipmentError;
    fn try_from(v: TomlSupportVersionReq) -> EquipmentResult<Self> {
        Ok(Self::try_from(v.version)?)
    }
}

impl RepositoryTrait for GitRepository {
    #[inline]
    fn kind(&self) -> RepositoryKind { RepositoryKind::Git }
    #[inline]
    fn provider(&self) -> &Key { &self.provider }
}