use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiKey(pub Key, pub Key);

impl TryFrom<String> for BiKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        v.split_once('/')
            .ok_or_else(|| EquipmentError::String { str: v.to_string(), err: StrErr::Path })
            .map(|(a,b)| Ok(Self(Key::try_from(a)?, Key::try_from(b)?)))?
    }
}

impl TryFrom<(&str,&str)> for BiKey {
    type Error = EquipmentError;
    fn try_from(v: (&str,&str)) -> EquipmentResult<Self> {
        Ok(Self(Key::try_from(v.0)?, Key::try_from(v.1)?))
    }
}

impl FromFixed<(&'static str, &'static str)> for BiKey {
    fn fixed(v: (&'static str, &'static str)) -> Self {
        Self::try_from((v.0, v.1)).expect("valid")
    }
}
