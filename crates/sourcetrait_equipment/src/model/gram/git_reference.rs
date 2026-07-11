use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReference(pub String);

impl TryFrom<String> for GitReference {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Ok(Self(v))
    }
}

impl From<&str> for GitReference {
    fn from(v: &str) -> Self { 
        Self::try_from(v.to_string()).expect("valid")
    }
}
