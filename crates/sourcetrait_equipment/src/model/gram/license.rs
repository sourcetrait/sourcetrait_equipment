use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct License {
    pub spdx: Option<Spdx>,
    pub file: PathBuf,
}
