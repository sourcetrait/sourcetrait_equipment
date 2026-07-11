use crate::*;

pub trait FromFixed<T>: Sized {
    fn fixed(v: T) -> Self;
}

pub trait RepositoryTrait: Clone + PartialEq {
    fn kind(&self) -> RepositoryKind;
    fn provider(&self) -> &Key;
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BagKey {
    Tri(TriKey),
    Quad(QuadKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: Key,
    pub title: Title,
    pub email: Email,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyword(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Details {
    pub summary: Summary,
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct License {
    pub spdx: Option<Spdx>,
    pub file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(pub semver::Version);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq(pub semver::VersionReq);

#[derive(Debug, Clone, PartialEq)]
pub struct Spdx(pub spdx::Expression);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryKind {
    Git,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericRepositorySet<REPO: RepositoryTrait> {
    pub key: TriKey,
    pub read: Option<REPO>,
    pub write: Option<REPO>,
    pub store: Option<REPO>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepositorySet {
    Git(GenericRepositorySet<GitRepository>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Repository {
    Git(GitRepository),
    Ipfs(IpfsRepository),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReference(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitRepository {
    pub provider: Key,
    pub uri: GitUri,
    pub reference: GitReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitUri {
    Https(url::Url),
    Git(url::Url),
    Ssh(url::Url),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpfsRepository {
    pub provider: Key,
    pub ipns: IpnsName,
    pub cid: IpfsCid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpfsCid(pub [u8; 32]);

impl TryFrom<&str> for IpfsCid {
    type Error = EquipmentError;

    fn try_from(s: &str) -> EquipmentResult<Self> {
        let bytes = base32::multi::Rfc4648::try_from(s)
            .and_then(|base32| base32.try_into_bytes())
            .map_err(|e| EquipmentError::String { str: s.to_string(), err: StrErr::Encoding })?;
            
        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpnsName(pub [u8; 36]);

impl TryFrom<&str> for IpnsName {
    type Error = EquipmentError;

    fn try_from(s: &str) -> EquipmentResult<Self> {
        let bytes = base36::Multi::try_from(s)
            .and_then(|base36| base36.try_into_bytes())
            .map_err(|e| EquipmentError::String { str: s.to_string(), err: StrErr::Encoding })?;
            
        Ok(Self(bytes))
    }
}
