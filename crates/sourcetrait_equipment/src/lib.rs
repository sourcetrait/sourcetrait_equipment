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
        pub(crate) mod export;
        pub(crate) mod equipment_imports;
        pub(crate) mod equipment_support;
        pub(crate) mod git_reference;
        pub(crate) mod git_repository;
        pub(crate) mod git_uri;
        pub(crate) mod ipfs_cid;
        pub(crate) mod ipfs_repository;
        pub(crate) mod ipns_name;
        pub(crate) mod key;
        pub(crate) mod keyword;
        pub(crate) mod license;
        pub(crate) mod quad_key;
        pub(crate) mod repository;
        pub(crate) mod repository_kind;
        pub(crate) mod repository_set;
        pub(crate) mod import;
        pub(crate) mod rig;
        pub(crate) mod spdx;
        pub(crate) mod summary;
        pub(crate) mod title;
        pub(crate) mod tri_key;
        pub(crate) mod version;
        pub(crate) mod version_req;
    }
    pub(crate) mod toml {
        pub(crate) mod author;
        pub(crate) mod common;
        pub(crate) mod details;
        pub(crate) mod equipment;
        pub(crate) mod equipment_exports;
        pub(crate) mod equipment_support;
        pub(crate) mod git_repository;
        pub(crate) mod ipfs_repository;
        pub(crate) mod license;
        pub(crate) mod repository_set;
        pub(crate) mod rig;
        pub(crate) mod import;
        pub(crate) mod imports;
        pub(crate) mod version_support;
    }
    pub(crate) mod traits;
}

pub use crate::{
    model::{
        gram::{
            author::Author,
            bag_key::BagKey,
            bi_key::BiKey,
            details::Details,
            email::Email,
            equipment::Equipment,
            export::{ExportEnum, RigExport, BagExport, GearBoxExport, GearDeskExport, GearLibExport},
            equipment_imports::EquipmentImports,
            equipment_support::EquipmentSupport,
            git_reference::GitReference,
            git_repository::GitRepository,
            git_uri::GitUri,
            ipfs_cid::IpfsCid,
            ipfs_repository::IpfsRepository,
            ipns_name::IpnsName,
            key::Key,
            keyword::Keyword,
            license::License,
            quad_key::QuadKey,
            repository::RepositoryEnum,
            repository_kind::RepositoryKind,
            repository_set::{RepositorySet, RepositorySetEnum},
            rig::Rig,
            import::{ImportEnum, RigImport, BagImport, GearBoxImport, GearDeskImport, GearLibImport},
            spdx::Spdx,
            summary::Summary,
            title::Title,
            tri_key::TriKey,
            version::Version,
            version_req::VersionReq,
        },
        toml::{
            author::AuthorToml,
            details::DetailsToml,
            equipment::EquipmentToml,
            equipment_exports::EquipmentExportsToml,
            equipment_support::EquipmentSupportToml,
            git_repository::GitRepositoryToml,
            ipfs_repository::IpfsRepositoryToml,
            license::LicenseToml,
            repository_set::{RepositorySetToml, RepositorySetEnumToml},
            rig::RigToml,
            import::{RigImportToml, GearBoxImportToml, GearDeskImportToml, GearLibImportToml, BagImportToml},
            imports::{ImportsToml, GearImportsToml},
            version_support::VersionSupportToml,
        },
        traits::{FromFixed, RepositoryTrait},
    },
    check::{StrCase, StrCheck},
    error::{EquipmentError, EquipmentResult, IoErr, StrErr},
};

pub use generated::ai::{
    AnnotatedResult
};

pub mod prelude {
    pub use crate::{
        AnnotatedResult, FromFixed,
    };
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

