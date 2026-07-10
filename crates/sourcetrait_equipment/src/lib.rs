pub(crate) mod generated {
    pub(crate) mod ai;
}
pub(crate) mod check;
pub(crate) mod error;
pub(crate) mod model {
    pub(crate) mod imp {
        pub(crate) mod common;
        pub(crate) mod rig;
        pub(crate) mod equipment;
    }
    pub(crate) mod toml {
        pub(crate) mod imp {
            pub(crate) mod rig;
        }
        pub(crate) mod common;
        pub(crate) mod rig;
        pub(crate) mod equipment;
    }
    pub(crate) mod common;
    pub(crate) mod rig;
    pub(crate) mod equipment;
}

pub use crate::{
    model::{
        imp::{
            common::{
                RepositoryTrait,
            },
        },
        common::{
            Key, BiKey, TriKey, Title, Details, License, Author, Version,
            Email, Summary, Keyword, Spdx, RepositoryKind, GenericRepositorySet,
            VersionReq, GitReference, GitUri, RepositorySet, GitRepository,
        },
        rig::{
            Rig, RigImports, RigImport,
        },
        equipment::{
            Equipment, EquipmentExports, EquipmentSupport,
        },
        toml::{
            common::{
                TomlAuthor, TomlProvider, TomlDetails, TomlLicense,
                TomlSupportVersionReq, TomlProviderMirror, TomlProviderContribute,
            },
            rig::{
                LibraryToml, LibraryTomlImports, LibraryTomlImport,
            },
            equipment::{
                EquipmentToml, EquipmentTomlSupport, EquipmentExportsToml,
            },
        },
    },
    check::{StrCase, StrCheck},
    error::{EquipmentError, EquipmentResult, IoErr, StrErr},
};

pub use generated::ai::{
    AnnotatedResult
};

pub mod prelude {
    pub use crate::AnnotatedResult;
}

pub(crate) use generated::ai::{
    BaseEncDec,
    base32,
    base36,
};

#[allow(unused)]
pub(crate) use std::{
    io,
    fs,
    fmt,
    str::FromStr,
    path::{Path,PathBuf},
};

