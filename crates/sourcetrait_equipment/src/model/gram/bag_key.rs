use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BagKey {
    Tri(TriKey),
    Quad(QuadKey),
}

impl TryFrom<String> for BagKey {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        match v.split('/').collect::<Vec<_>>()[..] {
            [a,b,c,d] => Ok(Self::Quad(QuadKey::try_from((a,b,c,d))?)),
            [a,b,c] => Ok(Self::Tri(TriKey::try_from((a,b,c))?)),
            _ => Err(EquipmentError::String { str: v, err: StrErr::Path }),
        }
    }
}
