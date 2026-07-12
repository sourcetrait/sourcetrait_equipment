use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RepositorySet<REPO: RepositoryTrait> {
    pub key: TriKey,
    pub read: Option<REPO>,
    pub write: Option<REPO>,
    pub store: Option<REPO>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepositorySetEnum {
    Git(RepositorySet<GitRepository>),
    Ipfs(RepositorySet<IpfsRepository>),
}

impl From<RepositorySet<GitRepository>> for RepositorySetEnum {
    fn from(v: RepositorySet<GitRepository>) -> Self {
        Self::Git(v)
    }
}

impl From<RepositorySet<IpfsRepository>> for RepositorySetEnum {
    fn from(v: RepositorySet<IpfsRepository>) -> Self {
        Self::Ipfs(v)
    }
}
