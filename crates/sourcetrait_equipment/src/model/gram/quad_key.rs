use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuadKey(pub Key, pub Key, pub Key, pub Key);

impl TryFrom<String> for QuadKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let [a,b,c,d] = v.split('/').collect::<Vec<_>>()[..] else {
            return Err(EquipmentError::String { str: v, err: StrErr::Path });
        };
        
        Ok(Self::try_from((a,b,c,d))?)
    }
}

impl TryFrom<(&str,&str,&str,&str)> for QuadKey {
    type Error = EquipmentError;
    fn try_from(v: (&str, &str, &str, &str)) -> EquipmentResult<Self> {
        Ok(Self(
            Key::try_from(v.0.to_string())?,
            Key::try_from(v.1.to_string())?,
            Key::try_from(v.2.to_string())?,
            Key::try_from(v.3.to_string())?,
        ))
    }
}
