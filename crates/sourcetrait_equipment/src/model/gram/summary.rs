use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary(pub String);

impl Summary {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT.max_len(120);
}

impl TryFrom<String> for Summary {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::CHECK.validate(v).map(|v| Self(v))
    }
}

impl FromFixed<&'static str> for Summary {
    fn fixed(v: &'static str) -> Self {
        Self::CHECK.validate(v).map(|v| Self(v.into())).expect("valid")
    }
}
