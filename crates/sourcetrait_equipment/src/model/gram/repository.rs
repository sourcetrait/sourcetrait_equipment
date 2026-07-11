use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Repository {
    Git(GitRepository),
    Ipfs(IpfsRepository),
}
