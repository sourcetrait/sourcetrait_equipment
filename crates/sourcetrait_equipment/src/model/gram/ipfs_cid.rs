use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpfsCid(pub [u8; 32]);

impl TryFrom<&str> for IpfsCid {
    type Error = EquipmentError;

    fn try_from(s: &str) -> EquipmentResult<Self> {
        let bytes = base32::multi::Rfc4648::try_from(s)
            .and_then(|base32| base32.try_into_bytes())
            .map_err(|e| EquipmentError::String { str: s.to_string(), err: StrErr::Encoding })?;
            
        Ok(Self(bytes))
    }
}
