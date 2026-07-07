pub(crate) mod check;
pub(crate) mod error;
pub(crate) mod model {
    pub(crate) mod common;
    pub(crate) mod library;
    pub(crate) mod rig;
    pub(crate) mod toml {
        pub(crate) mod common;
        pub(crate) mod library;
        pub(crate) mod rig;
    }
}

pub use crate::{
    check::{StrCase, StrCheck},
    error::{EquipmentError, EquipmentResult, IoErr, StrErr},
    model::{
        common::{
            Key, Title, Description, Provider, License, Author, Version,
            LibraryName, GitHubProvider, Email, Summary, Keyword, Spdx,
            VersionReq,
        },
        library::{
            Library, LibraryImports, LibraryImport,
        },
        rig::{
            Rig, RigExports, RigNushell,
        },
        toml::{
            common::{
                TomlAuthor, TomlProvider, TomlDescription, TomlLicense,
            },
            library::{
                LibraryToml, LibraryTomlImports, LibraryTomlImport,
            },
            rig::{
                RigToml, RigTomlNushell, RigTomlExports,
            },
        },
    },
};

pub(crate) use std::{
    io,
    fs,
    path::{Path,PathBuf},
};
