use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: Key,
    pub title: Title,
    pub email: Email,
}

impl TryFrom<TomlAuthor> for Author {
    type Error = EquipmentError;
    fn try_from(v: TomlAuthor) -> EquipmentResult<Self> {
        Ok(Self {
            name: Key::try_from(v.key)?,
            title: Title::try_from(v.title)?,
            email: Email::try_from(v.email)?,
        })
    }
}
