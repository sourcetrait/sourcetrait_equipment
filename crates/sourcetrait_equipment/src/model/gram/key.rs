use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key(pub String);

impl Key {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .case(StrCase::Snake)
        .max_len(16);
}

impl TryFrom<String> for Key {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}

impl TryFrom<&str> for Key {
    type Error = EquipmentError;
    fn try_from(v: &str) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v.into()))
    }
}

impl FromFixed<&'static str> for Key {
    fn fixed(v: &'static str) -> Self {
        Self::try_from(v).expect("valid")
    }
}
