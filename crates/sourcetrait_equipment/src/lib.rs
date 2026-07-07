pub(crate) mod check;
pub(crate) mod error;
pub(crate) mod model {
    pub(crate) mod imp {
        pub(crate) mod common;
        pub(crate) mod library;
        pub(crate) mod rig;
    }
    pub(crate) mod toml {
        pub(crate) mod imp {
            pub(crate) mod library;
        }
        pub(crate) mod common;
        pub(crate) mod library;
        pub(crate) mod rig;
    }
    pub(crate) mod common;
    pub(crate) mod library;
    pub(crate) mod rig;
}

pub use crate::{
    check::{StrCase, StrCheck},
    error::{EquipmentError, EquipmentResult, IoErr, StrErr},
    model::{
        common::{
            Key, BiKey, Title, Details, Provider, License, Author, Version,
            LibraryName, Email, Summary, Keyword, Spdx,
            VersionReq, ProviderAccount, ProviderContribute, ProviderMirror,
            ProviderReference, ProviderUri,
        },
        library::{
            Library, LibraryImports, LibraryImport,
        },
        rig::{
            Rig, RigExports, RigNushell,
        },
        toml::{
            common::{
                TomlAuthor, TomlProvider, TomlDetails, TomlLicense,
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
