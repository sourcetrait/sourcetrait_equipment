use std::{path::PathBuf};
use sourcetrait_equipment::{self as equipment, prelude::*};
use sourcetrait_testing::prelude::*;
use pretty_assertions::{assert_eq};

static TESTING: testing::Module = testing::module!(Integration, {
    .using_fixture_dir()
});

#[tested]
fn test_read() {
    let test = testing::test!({
        .inherit_fixture_dir()
    });

    let expected: equipment::Rig = equipment::Rig {
        key: equipment::Key::from("empower"),
        title: equipment::Title::from("SourceTrait Empower"),
        version: equipment::Version::from("0.0.0-2"),
        provider: equipment::Provider {
            key: equipment::BiKey::from(("github", "sourcetrait")),
            reference: equipment::ProviderReference::from("dev"),
            mirror: equipment::ProviderMirror {
                uri: equipment::ProviderUri::from("https://github.com/sourcetrait"),
            },
            contribute: equipment::ProviderContribute {
                uri: equipment::ProviderUri::from("ssh://git@github.com/sourcetrait"),
            },
        },
        author: equipment::Author {
            name: equipment::Key::from("sourcetrait"),
            title: equipment::Title::from("SourceTrait"),
            email: equipment::Email::from("development@sourcetrait.com"),
        },
        details: equipment::Details {
            summary: equipment::Summary::from("Empower rig"),
            keywords: Vec::from([
                equipment::Keyword::from("ai"),
            ]),
        },
        license: equipment::License {
            spdx: Some(equipment::Spdx::from("AGPL-3.0-or-later")),
            file: PathBuf::from("LICENSE-AGPL-3.txt"),
        },
        exports: equipment::RigExports {
            libraries: Vec::from([
                equipment::BiKey::from(("sourcetrait", "ant")),
                equipment::BiKey::from(("sourcetrait", "drone")),
                equipment::BiKey::from(("sourcetrait", "equip")),
                equipment::BiKey::from(("sourcetrait", "empower")),
                equipment::BiKey::from(("sourcetrait", "fae")),
                equipment::BiKey::from(("sourcetrait", "queen")),
            ]),
        },
        support: equipment::RigSupport {
            nushell: equipment::SupportVersionReq::from("0.113"),
            equipment: equipment::SupportVersionReq::from("0"),
        }, 
    };
    
    let actual = equipment::Rig::read(
        test.fixture_dir().join(equipment::RigToml::RIG_TOML)
    ).annotated().expect("TOML");

    assert_eq!(expected, actual);
}