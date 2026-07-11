use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpfsRepository {
    pub provider: Key,
    pub ipns: IpnsName,
    pub cid: IpfsCid,
}
