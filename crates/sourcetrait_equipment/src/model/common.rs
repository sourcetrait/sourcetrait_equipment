use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: Key,
    pub title: Title,
    pub email: Email,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyword(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Details {
    pub summary: Summary,
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct License {
    pub spdx: Option<Spdx>,
    pub file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(pub semver::Version);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq(pub semver::VersionReq);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    pub name: Key,
    pub account: ProviderAccount,
    pub reference: ProviderReference,
    pub mirror: ProviderMirror,
    pub contribute: ProviderContribute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAccount(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReference(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderMirror {
    pub uri: ProviderUri,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderContribute {
    pub uri: ProviderUri,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderUri {
    Http(String),
    Git(String),
    Ssh(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct Spdx(pub spdx::Expression);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryName(pub String);
