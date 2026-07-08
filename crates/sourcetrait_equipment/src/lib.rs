pub(crate) mod generated {
    pub(crate) mod ai;
}
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
    model::{
        common::{
            Key, BiKey, Title, Details, Provider, License, Author, Version,
            Email, Summary, Keyword, Spdx, SupportVersionReq,
            VersionReq, ProviderContribute, ProviderMirror,
            ProviderReference, ProviderUri,
        },
        library::{
            Library, LibraryImports, LibraryImport,
        },
        rig::{
            Rig, RigExports, RigSupport,
        },
        toml::{
            common::{
                TomlAuthor, TomlProvider, TomlDetails, TomlLicense,
                TomlSupportVersionReq, TomlProviderMirror, TomlProviderContribute,
            },
            library::{
                LibraryToml, LibraryTomlImports, LibraryTomlImport,
            },
            rig::{
                RigToml, RigTomlSupport, RigTomlExports,
            },
        },
    },
    //generated::ai::error::annotated::{Annotated, AnnotatedError},
    check::{StrCase, StrCheck},
    error::{EquipmentError, EquipmentResult, IoErr, StrErr},
};

pub use generated::ai::{
    Annotated
};

pub mod prelude {
    pub use crate::Annotated;
}

pub(crate) use std::{
    io,
    fs,
    path::{Path,PathBuf},
};
