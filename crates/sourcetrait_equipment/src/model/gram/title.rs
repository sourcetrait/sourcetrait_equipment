use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(pub String);

impl Title {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(32);
}

impl From<&str> for Title { fn from(v: &str) -> Self { Self(v.to_string()) } }

impl TryFrom<String> for Title {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}
