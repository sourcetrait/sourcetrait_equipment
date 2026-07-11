use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitRepository {
    pub provider: Key,
    pub uri: GitUri,
    pub reference: GitReference,
}

impl RepositoryTrait for GitRepository {
    #[inline]
    fn kind(&self) -> RepositoryKind { RepositoryKind::Git }
    #[inline]
    fn provider(&self) -> &Key { &self.provider }
}