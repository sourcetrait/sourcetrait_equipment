use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LicenseToml {
    pub spdx: Option<String>,
    pub file: PathBuf,
}

impl TryFrom<LicenseToml> for License {
    type Error = EquipmentError;
    fn try_from(v: LicenseToml) -> EquipmentResult<Self> {
        Ok(Self {
            spdx: v.spdx.map(|v| Spdx::try_from(v)).transpose()?,
            file: v.file,
        })
    }
}
