use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitUri {
    Https(url::Url),
    Git(url::Url),
    Ssh(url::Url),
}
impl GitUri {
    pub(crate) const HTTPS: &'static str = "https";
    pub(crate) const SSH: &'static str = "ssh";
}

impl TryFrom<String> for GitUri {
    type Error = EquipmentError;
    fn try_from(v: String) -> EquipmentResult<Self> {
        let url = url::Url::parse(&v)
            .map_err(|e| EquipmentError::url(e, v))?;

        match url.scheme() {
            GitUri::HTTPS => Ok(Self::Https(url)),
            GitUri::SSH => Ok(Self::Ssh(url)),
            p => Err(EquipmentError::ProviderProtocol { protocol: Some(p.to_string()) }),
        }
    }
}

impl FromFixed<&'static str> for GitUri {
    fn fixed(v: &str) -> Self {
        Self::try_from(v.to_string()).expect("valid")
    }
}
