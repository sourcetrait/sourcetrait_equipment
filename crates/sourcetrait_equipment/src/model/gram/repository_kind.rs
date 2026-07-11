use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryKind {
    Git,
    Ipfs,
}
