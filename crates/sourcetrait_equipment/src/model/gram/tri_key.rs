use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriKey(pub Key, pub Key, pub Key);

impl TryFrom<String> for TriKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let [a,b,c] = v.split('/').collect::<Vec<_>>()[..] else {
            return Err(EquipmentError::String { str: v, err: StrErr::Path });
        };
        
        Ok(Self::try_from((a,b,c))?)
    }
}

impl TryFrom<(&str,&str,&str)> for TriKey {
    type Error = EquipmentError;
    fn try_from(v: (&str, &str, &str)) -> EquipmentResult<Self> {
        Ok(Self(
            Key::try_from(v.0.to_string())?,
            Key::try_from(v.1.to_string())?,
            Key::try_from(v.2.to_string())?,
        ))
    }
}
