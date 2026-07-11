use crate::*;

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
    Ipfs(GenericRepositorySet<IpfsRepository>),
}
