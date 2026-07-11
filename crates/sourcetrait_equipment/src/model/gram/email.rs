use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl Email {
    pub(crate) const CHECK: StrCheck = StrCheck::DEFAULT
        .min_len(3)
        .max_len(255);

    pub fn check(s: &str) -> EquipmentResult<()> {
        Self::CHECK.check(s)?;
        
        let (local, domain) = {
            let mut parts = s.split('@');
            let local = parts.next()
                .ok_or_else(|| EquipmentError::Email { address: s.to_string() })?;
            let domain = parts.next()
                .ok_or_else(|| EquipmentError::Email { address: s.to_string() })?;
            if parts.count() != 0 {
                return Err(EquipmentError::Email { address: s.to_string() });
            }
            
            (local, domain)
        };
        
        if local.len() < 1 || domain.len() < 1 {
            return Err(EquipmentError::Email { address: s.to_string() });
        }

        Ok(())
    }

    pub(crate) fn valid(s: String) -> EquipmentResult<Self> {
        Self::check(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<String> for Email {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        Self::valid(v)
    }
}

impl From<&str> for Email {
    fn from(v: &str) -> Self {
        Self::try_from(v.to_string()).expect("valid email")
    }
}
