pub(crate) mod generated {
    pub(crate) mod ai;
}
pub(crate) mod check;
pub(crate) mod error;
pub(crate) mod model {
    pub(crate) mod gram {
        pub(crate) mod author;
        pub(crate) mod bag_key;
        pub(crate) mod bi_key;
        pub(crate) mod details;
        pub(crate) mod email;
        pub(crate) mod equipment;
        pub(crate) mod equipment_exports;
        pub(crate) mod equipment_imports;
        pub(crate) mod equipment_support;
        pub(crate) mod git_reference;
        pub(crate) mod git_repository;
        pub(crate) mod git_uri;
        pub(crate) mod ipfs_cid;
        pub(crate) mod ipfs_repository;
        pub(crate) mod ipns_name;
        pub(crate) mod key;
        pub(crate) mod license;
        pub(crate) mod quad_key;
        pub(crate) mod repository;
        pub(crate) mod repository_kind;
        pub(crate) mod repository_set;
        pub(crate) mod rig_import;
        pub(crate) mod rig_imports;
        pub(crate) mod rig;
        pub(crate) mod spdx;
        pub(crate) mod summary;
        pub(crate) mod title;
        pub(crate) mod tri_key;
        pub(crate) mod version;
        pub(crate) mod version_req;
    }
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
    pub(crate) mod traits;
}

pub use crate::{
    model::{
        gram::{
            key::Key,
            bi_key::BiKey,
            equipment::Equipment,
            equipment_exports::EquipmentExports,
            equipment_imports::EquipmentImports,
            equipment_support::EquipmentSupport,
            rig::Rig,
            rig_import::RigImport,
            rig_imports::RigImports,
        },
        traits::{
            FromFixed, RepositoryTrait,
            TriKey, QuadKey, BagKey, Title, Details, License, Author, Version,
            Email, Summary, Keyword, Spdx, RepositoryKind, GenericRepositorySet,
            VersionReq, GitReference, GitUri, RepositorySet, GitRepository,
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

