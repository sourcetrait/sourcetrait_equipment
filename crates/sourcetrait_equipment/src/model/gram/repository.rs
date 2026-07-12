use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryEnum {
    Git(GitRepository),
    Ipfs(IpfsRepository),
}
