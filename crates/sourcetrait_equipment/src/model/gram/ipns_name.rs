use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpnsName(pub [u8; 36]);

impl TryFrom<&str> for IpnsName {
    type Error = EquipmentError;

    fn try_from(s: &str) -> EquipmentResult<Self> {
        let bytes = base36::Multi::try_from(s)
            .and_then(|base36| base36.try_into_bytes())
            .map_err(|e| EquipmentError::String { str: s.to_string(), err: StrErr::Encoding })?;
            
        Ok(Self(bytes))
    }
}
