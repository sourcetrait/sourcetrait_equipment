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

    let expected: equipment::Equipment = equipment::Equipment {
        key: equipment::BiKey::fixed("empower"),
        title: equipment::Title::from("SourceTrait Empower"),
        version: equipment::Version::from("0.0.0-2"),
        repositories: equipment::Provider {
            key: equipment::BiKey::from(("github", "sourcetrait")),
            reference: equipment::GitReference::from("dev"),
            mirror: equipment::ProviderMirror {
                uri: equipment::GitUri::from("https://github.com/sourcetrait"),
            },
            contribute: equipment::ProviderContribute {
                uri: equipment::GitUri::from("ssh://git@github.com/sourcetrait"),
            },
        },
        author: equipment::Author {
            name: equipment::BiKey::from("sourcetrait"),
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
        exports: equipment::EquipmentExports {
            rig: Vec::from([
                equipment::BiKey::from(("sourcetrait", "ant")),
                equipment::BiKey::from(("sourcetrait", "drone")),
                equipment::BiKey::from(("sourcetrait", "equip")),
                equipment::BiKey::from(("sourcetrait", "empower")),
                equipment::BiKey::from(("sourcetrait", "fae")),
                equipment::BiKey::from(("sourcetrait", "queen")),
            ]),
        },
        support: equipment::EquipmentSupport {
            nushell: equipment::SupportVersionReq::from("0.113"),
            equipment: equipment::SupportVersionReq::from("0"),
        }, 
    };
    
    let actual = equipment::Equipment::read(
        test.fixture_dir().join(equipment::EquipmentToml::EQUIPMENT_TOML)
    ).annotated().expect("TOML");

    assert_eq!(expected, actual);
}