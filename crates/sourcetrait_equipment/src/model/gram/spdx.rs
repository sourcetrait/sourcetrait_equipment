use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Spdx(pub spdx::Expression);

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
