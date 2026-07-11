use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyword(pub String);

impl Keyword {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .max_len(8)
        .case(StrCase::Snake);
}

impl TryFrom<String> for Keyword {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}

impl From<&str> for Keyword {
    fn from(v: &str) -> Self {
        Self::CHECK.validate(v.to_string()).map(|v| Self(v)).expect("valid")
    }
}
