use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DetailsToml {
    pub summary: String,
    pub keywords: Vec<String>,
}

impl TryFrom<DetailsToml> for Details {
    type Error = EquipmentError;
    fn try_from(v: DetailsToml) -> EquipmentResult<Self> {
        Ok(Self {
            summary: Summary::try_from(v.summary)?,
            keywords: v.keywords.into_iter()
                .map(|k| Keyword::try_from(k))
                .collect::<EquipmentResult<_>>()?,
        })
    }
}
