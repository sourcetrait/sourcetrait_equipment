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

impl FromFixed<(&'static str, &'static str, &'static str, &'static str)> for QuadKey {
    fn fixed(v: (&'static str, &'static str, &'static str, &'static str)) -> Self {
        Self::try_from((v.0, v.1, v.2, v.3)).expect("valid")
    }
}
